use crate::core::geometry::CsgCompiler;
use crate::core::world::{EditorMode, EditorWorld};
use bevy::asset::RenderAssetUsages;
use bevy::camera::primitives::Aabb;
use bevy::math::bounding::{Aabb3d, RayCast3d};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use manifold_csg::OpType;

#[derive(Component)]
pub struct HoveredBrush;

#[derive(Component)]
pub struct WorkshopBrushMarker {
    pub brush_id: usize,
}

#[derive(Component)]
pub struct GlobalManifoldMarker;

pub struct RenderEnginePlugin;

impl Plugin for RenderEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::pbr::wireframe::WireframePlugin::default())
            .add_systems(Startup, setup_render_environment)
            .add_systems(Update, rebuild_dirty_world_system)
            .add_systems(Update, brush_hover_picking_system)
            .add_systems(Update, visual_highlight_system);
    }
}

fn setup_render_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        Visibility::Visible,
        InheritedVisibility::default(),
        GlobalManifoldMarker,
    ));
}

fn update_mesh_geometry(bevy_mesh: &mut Mesh, mesh_gl: &manifold_csg::MeshGL) {
    let stride = mesh_gl.num_prop() as usize;
    let positions: Vec<[f32; 3]> = mesh_gl
        .vert_properties()
        .chunks_exact(stride)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect();

    let indices: Vec<u32> = mesh_gl.tri_verts();

    bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);
    bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
    bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_TANGENT);

    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    bevy_mesh.insert_indices(Indices::U32(indices));

    let _ = bevy_mesh.try_duplicate_vertices();
    let _ = bevy_mesh.compute_flat_normals();
}

fn clear_mesh_geometry(bevy_mesh: &mut Mesh) {
    bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
    bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);
    bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
    bevy_mesh.remove_attribute(Mesh::ATTRIBUTE_TANGENT);
    bevy_mesh.remove_indices();
}

fn rebuild_dirty_world_system(
    mut commands: Commands,
    mut world_state: ResMut<EditorWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_mesh_query: Query<(&Mesh3d, &mut Visibility), With<GlobalManifoldMarker>>,
    workshop_mesh_query: Query<(Entity, &Mesh3d, &WorkshopBrushMarker)>,
) {
    if !world_state.needs_rebuild {
        return;
    }

    match world_state.editor_mode {
        EditorMode::ManifoldMode => {
            for (entity, _, _) in &workshop_mesh_query {
                commands.entity(entity).despawn();
            }

            if let Ok((mesh3d, mut visibility)) = global_mesh_query.single_mut() {
                // Access the internal handle from Mesh3d using .0
                if let Some(bevy_mesh) = meshes.get_mut(&mesh3d.0) {
                    if let Some(mesh_gl) = CsgCompiler::compile_world(&world_state) {
                        update_mesh_geometry(bevy_mesh, &mesh_gl);
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }

        EditorMode::WorkshopMode => {
            if let Ok((_, mut visibility)) = global_mesh_query.single_mut() {
                *visibility = Visibility::Hidden;
            }

            let mut updated_brush_ids = std::collections::HashSet::new();

            for (entity, mesh_handle, marker) in &workshop_mesh_query {
                if let Some(b) = world_state.brushes.get(marker.brush_id) {
                    if let Some(bevy_mesh) = meshes.get_mut(&mesh_handle.0) {
                        update_mesh_geometry(bevy_mesh, &b.cached_meshgl);
                    }

                    updated_brush_ids.insert(marker.brush_id);
                } else {
                    commands.entity(entity).despawn();
                }
            }

            for (index, b) in world_state.brushes.iter().enumerate() {
                if updated_brush_ids.contains(&index) {
                    continue;
                }

                let mut new_mesh = Mesh::new(
                    bevy::render::render_resource::PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default(),
                );
                update_mesh_geometry(&mut new_mesh, &b.cached_meshgl);

                let brush_material = match b.mode {
                    OpType::Subtract => StandardMaterial {
                        base_color: Color::srgba(0.9, 0.2, 0.2, 0.4),
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: None,
                        depth_bias: 10.0,
                        ..default()
                    },
                    OpType::Add => StandardMaterial {
                        base_color: Color::srgba(0.2, 0.8, 0.2, 0.4),
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: None,
                        depth_bias: 0.0,
                        ..default()
                    },
                    OpType::Intersect => StandardMaterial {
                        base_color: Color::srgba(1.0, 0.6, 0.0, 0.4),
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: None,
                        depth_bias: 5.0,
                        ..default()
                    },
                };

                let mesh_bundle = (
                    Mesh3d(meshes.add(new_mesh)),
                    MeshMaterial3d(materials.add(brush_material)),
                    Transform::IDENTITY,
                    Visibility::Visible,
                    InheritedVisibility::default(),
                    WorkshopBrushMarker { brush_id: index },
                );

                commands.spawn(mesh_bundle);

                // if b.mode == OpType::Subtract {
                //     commands.spawn((
                //         mesh_bundle,
                //         bevy::pbr::wireframe::Wireframe,
                //         bevy::pbr::wireframe::WireframeColor {
                //             color: Color::srgb(1.0, 0.0, 0.0), // High visibility red outlines
                //         },
                //     ));
                // } else {
                //     commands.spawn(mesh_bundle);
                // }
            }
        }
    }

    world_state.needs_rebuild = false;
}

fn brush_hover_picking_system(
    mut commands: Commands,
    window: Single<&Window>,
    camera_data: Single<(&Camera, &GlobalTransform)>,
    brush_query: Query<(Entity, &GlobalTransform, &Aabb), With<WorkshopBrushMarker>>,
    hovered_query: Query<Entity, With<HoveredBrush>>,
) {
    for entity in &hovered_query {
        commands.entity(entity).remove::<HoveredBrush>();
    }

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = *camera_data;
    let Ok(world_ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let mut closest_hit: Option<(Entity, f32)> = None;

    for (entity, global_transform, local_aabb) in &brush_query {
        let world_to_local = global_transform.affine().inverse();

        let local_origin = world_to_local.transform_point3(world_ray.origin);
        let local_direction = world_to_local
            .transform_vector3(world_ray.direction.to_vec3a().to_vec3())
            .normalize();

        let Ok(local_dir3) = Dir3::new(local_direction) else {
            continue;
        };

        let local_ray = Ray3d::new(local_origin, local_dir3);
        let ray_cast = RayCast3d::from_ray(local_ray, f32::MAX);

        let local_aabb_3d = Aabb3d::new(
            Vec3::from(local_aabb.center),
            Vec3::from(local_aabb.half_extents),
        );

        if let Some(distance) = ray_cast.aabb_intersection_at(&local_aabb_3d) {
            let hit_point_local = local_origin + local_direction * distance;
            let hit_point_world = global_transform.affine().transform_point3(hit_point_local);
            let world_distance = world_ray.origin.distance(hit_point_world);

            if closest_hit.is_none() || world_distance < closest_hit.unwrap().1 {
                closest_hit = Some((entity, world_distance));
            }
        }
    }

    if let Some((entity, _)) = closest_hit {
        commands.entity(entity).insert(HoveredBrush);
    }
}

fn visual_highlight_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Query brushes that are currently hovered
    hovered_brush_query: Query<
        (&MeshMaterial3d<StandardMaterial>, &WorkshopBrushMarker),
        With<HoveredBrush>,
    >,
    normal_brush_query: Query<
        (&MeshMaterial3d<StandardMaterial>, &WorkshopBrushMarker),
        Without<HoveredBrush>,
    >,
    world_state: Res<EditorWorld>,
) {
    for (material_handle, marker) in &hovered_brush_query {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color.set_alpha(0.8);
        }
    }

    for (material_handle, marker) in &normal_brush_query {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color.set_alpha(0.4);
        }
    }
}
