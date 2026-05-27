use gltf_json::accessor::GenericComponentType;
use gltf_json::validation::{Checked, USize64};
use std::fs::File;
use std::io::Write;

use crate::core::world::EditorWorld;

/// Computes the active Manifold CSG world tree and writes a specification-compliant glTF + bin bundle
pub fn export_world_to_gltf(world: &EditorWorld) {
    // 1. Accumulate the CSG tree identically to how the renderer generates it
    let mut runtime_manifold = manifold_csg::Manifold::empty();
    let brushes_clone = world.brushes.clone();
    for br in brushes_clone {
        runtime_manifold = runtime_manifold.boolean(&br.manifold, br.mode);
    }

    // 2. Fetch raw geometry data arrays
    let (vert_props, _, tri_indices) = runtime_manifold.to_mesh_f32();
    if vert_props.is_empty() || tri_indices.is_empty() {
        println!("Export cancelled: The baked layout doesn't contain any geometry!");
        return;
    }

    let mut positions: Vec<f32> = Vec::new();
    let stride = 3;
    for chunk in vert_props.chunks_exact(stride) {
        positions.push(chunk[0]);
        positions.push(chunk[1]);
        positions.push(chunk[2]);
    }

    let indices: Vec<u32> = tri_indices;

    // 3. Assemble binary buffer block arrays
    let mut buffer_bytes = Vec::new();

    // Position bytes setup - FIXED: Forcing Little-Endian (`to_le_bytes`) for specification compliance
    let pos_offset = buffer_bytes.len();
    for pos in &positions {
        buffer_bytes.extend_from_slice(&pos.to_le_bytes());
    }
    let pos_length = buffer_bytes.len() - pos_offset;

    // Indices bytes setup - FIXED: Forcing Little-Endian (`to_le_bytes`) for specification compliance
    let idx_offset = buffer_bytes.len();
    for idx in &indices {
        buffer_bytes.extend_from_slice(&idx.to_le_bytes());
    }
    let idx_length = buffer_bytes.len() - idx_offset;

    // 4. Construct structural glTF JSON node definitions
    let mut json = gltf_json::Root::default();

    let buffer_idx = json.push(gltf_json::Buffer {
        byte_length: USize64(buffer_bytes.len() as u64),
        name: None,
        uri: Some("pangu.bin".to_string()),
        extensions: None,
        extras: None,
    });

    let pos_view_idx = json.push(gltf_json::buffer::View {
        buffer: buffer_idx,
        byte_length: USize64(pos_length as u64),
        byte_offset: Some(USize64(pos_offset as u64)),
        byte_stride: None,
        name: None,
        target: Some(Checked::Valid(gltf_json::buffer::Target::ArrayBuffer)),
        extensions: None,
        extras: None,
    });

    let idx_view_idx = json.push(gltf_json::buffer::View {
        buffer: buffer_idx,
        byte_length: USize64(idx_length as u64),
        byte_offset: Some(USize64(idx_offset as u64)),
        byte_stride: None,
        name: None,
        target: Some(Checked::Valid(
            gltf_json::buffer::Target::ElementArrayBuffer,
        )),
        extensions: None,
        extras: None,
    });

    let pos_accessor_idx = json.push(gltf_json::Accessor {
        buffer_view: Some(pos_view_idx),
        byte_offset: Some(USize64(0)),
        count: USize64((positions.len() / 3) as u64),
        component_type: Checked::Valid(GenericComponentType(
            gltf_json::accessor::ComponentType::F32,
        )),
        extensions: None,
        extras: None,
        type_: Checked::Valid(gltf_json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let idx_accessor_idx = json.push(gltf_json::Accessor {
        buffer_view: Some(idx_view_idx),
        byte_offset: Some(USize64(0)),
        count: USize64(indices.len() as u64),
        component_type: Checked::Valid(GenericComponentType(
            gltf_json::accessor::ComponentType::U32,
        )),
        extensions: None,
        extras: None,
        type_: Checked::Valid(gltf_json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let primitive = gltf_json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(
                Checked::Valid(gltf_json::mesh::Semantic::Positions),
                pos_accessor_idx,
            );
            map
        },
        extensions: None,
        extras: None,
        indices: Some(idx_accessor_idx),
        material: None,
        mode: Checked::Valid(gltf_json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh_idx = json.push(gltf_json::Mesh {
        extensions: None,
        extras: None,
        name: Some("BakedLevelMesh".to_string()),
        primitives: vec![primitive],
        weights: None,
    });

    let node_idx = json.push(gltf_json::Node {
        camera: None,
        children: None,
        extensions: None,
        extras: None,
        matrix: None,
        mesh: Some(mesh_idx),
        name: Some("Level_Root".to_string()),
        rotation: None,
        scale: None,
        translation: None,
        weights: None,
        skin: None,
    });

    json.push(gltf_json::Scene {
        extensions: None,
        extras: None,
        name: Some("Pangu_Scene".to_string()),
        nodes: vec![node_idx],
    });

    // 5. Commit data pools locally to storage files
    let json_string = gltf_json::serialize::to_string_pretty(&json).expect("Serialization failure");

    let mut gltf_file = File::create("pangu.gltf").expect("Unable to create pangu.gltf");
    gltf_file
        .write_all(json_string.as_bytes())
        .expect("Write failure on pangu.gltf");

    let mut bin_file = File::create("pangu.bin").expect("Unable to create pangu.bin");
    bin_file
        .write_all(&buffer_bytes)
        .expect("Write failure on pangu.bin");

    println!("SUCCESS: Little-Endian standard level exported as 'pangu.gltf' and 'pangu.bin'!");
}
