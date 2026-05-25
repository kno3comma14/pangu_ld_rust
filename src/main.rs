use std::time::Instant;

use bevy::audio::CpalSample;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use manifold_csg::Manifold;

mod core;
mod gui;
use crate::core::brush::{Brush, BrushMode, BrushPrimitive};
use crate::core::render::RenderEnginePlugin;
use crate::core::world::EditorWorld;

#[derive(Component)]
pub struct EditorGridMarker;

fn mouse_grid_raycast_system(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
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

    // Check height modification hotkeys
    let mut height_changed = false;
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        world_state.grid_height += snap;
        height_changed = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        world_state.grid_height -= snap;
        height_changed = true;
    }

    let current_grid_y = world_state.grid_height;

    // FIX: Using .single_mut() maps cleanly to modern Bevy's API expectations
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

            let cell_x = (intersection_point.x / snap).floor() * snap;
            let cell_z = (intersection_point.z / snap).floor() * snap;

            let render_height = current_grid_y + 0.01;
            let half_snap = snap * 0.5;
            let center_position = Vec3::new(cell_x + half_snap, render_height, cell_z + half_snap);

            gizmos.rect(
                Isometry3d::new(
                    center_position,
                    Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                ),
                Vec2::splat(snap),
                Color::srgb(1.0, 0.75, 0.0),
            );

            if mouse_input.just_pressed(MouseButton::Left) {
                let spawn_transform =
                    Transform::from_xyz(center_position.x, current_grid_y, center_position.z);

                let new_cube_brush = Brush::new(
                    BrushPrimitive::Cube {
                        size: Vec3::splat(2.0),
                    },
                    BrushMode::Subtractive,
                    spawn_transform,
                );

                world_state.add_brush(&new_cube_brush);
                world_state.needs_rebuild = true;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pangu Level Designer".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FreeCameraPlugin)
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(RenderEnginePlugin)
        .insert_resource(initialize_testing_world())
        .add_systems(Startup, setup_editor_workspace)
        .add_systems(Update, mouse_grid_raycast_system)
        .run();
}

fn initialize_testing_world() -> EditorWorld {
    let mut world = EditorWorld::new();

    let mut i: u32 = 0;
    let t0 = Instant::now();
    while i < 10000 {
        let bm = BrushMode::Additive;
        let base_cube = Brush::new(
            BrushPrimitive::Cube {
                size: Vec3::splat(1.0),
            },
            bm,
            Transform::IDENTITY.with_scale(vec3(6.0, 1.0, 6.0)),
        );
        world.add_brush(&base_cube);
        i += 1;
    }
    let elapsed = t0.elapsed();
    println!("elapsed: {:.3?}", elapsed);

    world
}

fn setup_editor_workspace(mut commands: Commands) {
    commands.spawn((InfiniteGridBundle::default(), EditorGridMarker));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 5.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera {
            mouse_key_cursor_grab: MouseButton::Right,
            ..default()
        },
    ));
}
