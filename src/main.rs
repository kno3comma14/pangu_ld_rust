use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy::window::Window;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};

mod core;
mod file;
mod gui;
mod tool;
use crate::core::brush::{Brush, BrushPrimitive};
use crate::core::render::RenderEnginePlugin;
use crate::core::world::EditorWorld;
use crate::gui::editor_mode_switcher::EditorModeSwitcherPlugin;
use crate::gui::gltf_exporter::GltfExporterGuiPlugin;
use crate::gui::grid::GridPlugin;
use crate::gui::primitive_selector::primitive_selector_system;

#[derive(Component)]
pub struct EditorGridMarker;

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
        .add_plugins(GltfExporterGuiPlugin)
        .add_plugins(EditorModeSwitcherPlugin)
        .add_plugins(GridPlugin)
        .insert_resource(initialize_world())
        .add_systems(Startup, setup_editor_workspace)
        .add_systems(Update, primitive_selector_system)
        .run();
}

fn initialize_world() -> EditorWorld {
    let mut world = EditorWorld::new();
    let bm = manifold_csg::OpType::Add;
    let base_cube = Brush::new(
        BrushPrimitive::Cube { size: Vec3::ONE },
        bm,
        Transform::IDENTITY.with_scale(Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        }),
    );
    world.add_brush(&base_cube);
    world.apply_brush(&base_cube);
    world
}

fn setup_editor_workspace(mut commands: Commands) {
    commands.spawn((InfiniteGridBundle::default(), EditorGridMarker));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(12.0, 15.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera {
            mouse_key_cursor_grab: MouseButton::Right,
            ..default()
        },
    ));
}
