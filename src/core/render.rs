use crate::core::geometry::CsgCompiler;
use crate::core::world::EditorWorld;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

#[derive(Component)]
pub struct WorldMeshMarker;

pub struct RenderEnginePlugin;

impl Plugin for RenderEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_render_environment)
            .add_systems(Update, rebuild_dirty_world_system);
    }
}

fn setup_render_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 600.0,
    //     affects_lightmapped_meshes: true,
    // });

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let empty_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let mesh_handle = meshes.add(empty_mesh);

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 0.9), // Classic blueprint developer blue
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::IDENTITY,
        WorldMeshMarker,
    ));
}

fn rebuild_dirty_world_system(
    mut world_state: ResMut<EditorWorld>,
    mesh_query: Query<&Mesh3d, With<WorldMeshMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !world_state.needs_rebuild {
        return;
    }

    if let Ok(mesh_handle) = mesh_query.single() {
        if let Some(bevy_mesh) = meshes.get_mut(mesh_handle) {
            if let Some(mesh_gl) = CsgCompiler::compile_world(&world_state) {
                let stride = mesh_gl.num_prop() as usize;
                let positions: Vec<[f32; 3]> = mesh_gl
                    .vert_properties()
                    .chunks_exact(stride)
                    .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                    .collect();

                let indices: Vec<u32> = mesh_gl.tri_verts();

                bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                bevy_mesh.insert_indices(Indices::U32(indices));

                bevy_mesh.duplicate_vertices();
                let _ = bevy_mesh.compute_flat_normals();
            } else {
                bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
                bevy_mesh.remove_indices();
            }
        }
    }

    world_state.needs_rebuild = false;
}
