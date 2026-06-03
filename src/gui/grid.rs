use crate::EditorGridMarker;
use crate::core::brush::Brush;
use crate::core::world::{EditorToolActive, EditorWorld};
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SnapIndex(pub usize);

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnapIndex(4))
            .add_systems(Update, mouse_grid_raycast_system)
            .add_systems(Update, change_grid_snap)
            .add_systems(Update, toggle_snap_usage);
    }
}

fn mouse_grid_raycast_system(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
    mut grid_query: Query<&mut Transform, With<EditorGridMarker>>, // Clean, direct query parameter
    mut gizmos: Gizmos,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let snap = world_state.grid_snap;

    let mut height_changed = false;
    if keyboard_input.just_pressed(KeyCode::KeyY) {
        world_state.grid_height += snap;
        height_changed = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        world_state.grid_height -= snap;
        height_changed = true;
    }

    let current_grid_y = world_state.grid_height;

    if height_changed {
        if let Ok(mut grid_transform) = grid_query.single_mut() {
            grid_transform.translation.y = current_grid_y;
        }
    }

    let Some(ray) = camera
        .viewport_to_world(camera_transform, cursor_position)
        .ok()
    else {
        return;
    };

    if ray.direction.y.abs() > 0.0001 {
        let t = (current_grid_y - ray.origin.y) / ray.direction.y;
        if t >= 0.0 {
            let intersection_point = ray.get_point(t);

            let cell_x = (intersection_point.x / snap).floor() * snap + 0.01;
            let cell_z = (intersection_point.z / snap).floor() * snap + 0.01;

            let render_height = current_grid_y + 0.01;
            let half_snap = snap * 0.5;
            let center_position = Vec3::new(cell_x + half_snap, render_height, cell_z + half_snap);
            world_state.central_mouse_coords = center_position;
            gizmos.rect(
                Isometry3d::new(
                    center_position,
                    Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                ),
                Vec2::splat(snap),
                Color::srgb(1.0, 0.75, 0.0),
            );

            if keyboard_input.just_pressed(KeyCode::Space)
                && world_state.active_tool == EditorToolActive::Add
            {
                let spawn_transform =
                    Transform::from_xyz(center_position.x, current_grid_y, center_position.z);

                let brush_to_spawn = Brush::new(
                    world_state.selected_brush_primitive,
                    world_state.add_brush_mode,
                    spawn_transform,
                );

                world_state.add_brush(&brush_to_spawn);
                world_state.apply_brush(&brush_to_spawn);
            }
        }
    }
}

fn change_grid_snap(
    mut world_state: ResMut<EditorWorld>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snap_index: ResMut<SnapIndex>,
) {
    let snap_values = vec![0.125, 0.25, 0.5, 0.75, 1.0];
    let max_idx = snap_values.len() - 1;

    if keyboard_input.just_pressed(KeyCode::BracketRight) {
        let current = snap_index.0 as isize + 1;
        snap_index.0 = current.clamp(0, max_idx as isize) as usize;
    } else if keyboard_input.just_pressed(KeyCode::BracketLeft) {
        let current = snap_index.0 as isize - 1;
        snap_index.0 = current.clamp(0, max_idx as isize) as usize;
    }

    world_state.grid_snap = snap_values[snap_index.0];
}

fn toggle_snap_usage() {}
