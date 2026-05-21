use bevy::camera_controller::pan_camera::{PanCamera, PanCameraPlugin};
use bevy::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};

mod core;
use crate::core::brush::{Brush, BrushMode, BrushPrimitive};
use crate::core::render::RenderEnginePlugin;
use crate::core::world::EditorWorld;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pangu Level Designer - Kernel Verification".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanCameraPlugin)
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(RenderEnginePlugin)
        .insert_resource(initialize_testing_world())
        .add_systems(Startup, setup_editor_workspace)
        .run();
}

fn initialize_testing_world() -> EditorWorld {
    let mut world = EditorWorld::new();

    let base_cube = Brush {
        primitive: BrushPrimitive::Cube {
            size: Vec3::splat(2.0),
        },
        mode: BrushMode::Additive,
        transform: Transform::IDENTITY,
    };
    world.add_brush(base_cube);

    let carving_sphere = Brush {
        primitive: BrushPrimitive::Sphere {
            radius: 1.3,
            subdivisions: 32,
        },
        mode: BrushMode::Subtractive,
        transform: Transform::IDENTITY.with_scale(vec3(0.9, 0.9, 0.9)),
    };
    world.add_brush(carving_sphere);

    world
}

fn setup_editor_workspace(mut commands: Commands) {
    commands.spawn(InfiniteGridBundle::default());

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 5.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanCamera {
            pan_speed: 15.0,
            zoom_speed: 0.05,
            rotation_speed: 1.0,
            ..default()
        },
    ));
}
