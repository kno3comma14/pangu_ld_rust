// use crate::core::world::EditorWorld;
// use bevy::camera::{Camera, Camera3d};
// use bevy::ecs::query::With;
// use bevy::gizmos::gizmos::Gizmos;
// use bevy::prelude::*;
// use bevy::window::{PrimaryWindow, Window};

// fn mouse_grid_raycast_system(
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
//     world_state: Res<EditorWorld>,
//     mut gizmos: Gizmos,
// ) {
//     // 1. Isolate the mouse pointer coordinates from the window canvas viewport
//     let Ok(window) = window_query.single() else {
//         return;
//     };
//     let Some(cursor_position) = window.cursor_position() else {
//         return;
//     };

//     // 2. Fetch the rendering camera transformation pipelines
//     let Ok((camera, camera_transform)) = camera_query.single() else {
//         return;
//     };

//     // 3. Transform the flat 2D screen pixels into a 3D structural Ray trajectory vector
//     let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
//         return;
//     };

//     // 4. Perform Ray-to-Plane intersection calculations against the Infinite Grid floor (Y = 0)
//     // Mathematical rule: Target point P = RayOrigin + t * RayDirection where P.y = 0.
//     // Therefore: 0 = Origin.y + t * Direction.y -> t = -Origin.y / Direction.y
//     if ray.direction.y.abs() > 0.0001 {
//         // Eliminate edge cases where camera points horizontally
//         let t = -ray.origin.y / ray.direction.y;

//         // If 't' is negative, the ground interception occurs behind the viewport lens camera
//         if t >= 0.0 {
//             let intersection_point = ray.origin + ray.direction * t;

//             // 5. Compute grid position bounds using your active snapping modifier (e.g., world.grid_snap = 1.0)
//             let snap = world_state.grid_snap;

//             // Calculate the exact bottom-left alignment floor corner anchor of the targeted cell
//             let cell_x = (intersection_point.x / snap).floor() * snap;
//             let cell_z = (intersection_point.z / snap).floor() * snap;

//             // 6. Draw the highlighted cell section box boundaries on screen using immediate-mode gizmos
//             // We shift the target plane line drawing slightly upwards on Y (e.g., 0.01) to eliminate
//             // visual Z-fighting flickering bugs with the underlying grid wire structures.
//             let render_height = 0.01;
//             let half_snap = snap * 0.5;
//             let center_position = Vec3::new(cell_x + half_snap, render_height, cell_z + half_snap);

//             // Renders a flat bounding square over the specific snapped coordinate target
//             gizmos.rect(
//                 center_position,
//                 Quat::from_rotation_x(std::f32::consts::FRAC_PI_2), // Rotate flat flush onto the XZ ground layout
//                 Vec2::splat(snap),
//                 Color::srgb(1.0, 0.75, 0.0), // Bright yellow workspace indicator highlight
//             );
//         }
//     }
// }
