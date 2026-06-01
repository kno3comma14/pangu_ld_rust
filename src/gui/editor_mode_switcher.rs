use crate::core::world::{EditorMode, EditorWorld};
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

pub struct EditorModeSwitcherPlugin;

impl Plugin for EditorModeSwitcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, switch_editor_mode);
    }
}

fn switch_editor_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        match world_state.editor_mode {
            EditorMode::WorkshopMode => {
                world_state.editor_mode = EditorMode::ManifoldMode;
            }
            EditorMode::ManifoldMode => {
                world_state.editor_mode = EditorMode::WorkshopMode;
            }
        }
        world_state.needs_rebuild = true;
    }
}
