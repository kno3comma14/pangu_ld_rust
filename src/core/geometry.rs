use crate::core::world::EditorWorld;
use bevy::transform::components::Transform;
use manifold_csg::MeshGL;

pub struct CsgCompiler;

impl CsgCompiler {
    pub fn compile_world(world: &EditorWorld) -> Option<MeshGL> {
        let (vert_props, n_props, tri_indices) = world.base_manifold.to_mesh_f32();
        let mesh_gl = MeshGL::new(&vert_props, n_props, &tri_indices);
        Some(mesh_gl)
    }
}

pub fn bevy_transform_to_affine(t: Transform) -> [f64; 12] {
    let cols = t.to_matrix().to_cols_array();
    let t = [
        cols[0] as f64,
        cols[1] as f64,
        cols[2] as f64,
        cols[4] as f64, // row 0
        cols[5] as f64,
        cols[6] as f64,
        cols[8] as f64,
        cols[9] as f64, // row 1
        cols[10] as f64,
        cols[12] as f64,
        cols[13] as f64,
        cols[14] as f64, // row 2
    ];
    t
}
