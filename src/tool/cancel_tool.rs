use crate::core::world::{EditorToolActive, EditorWorld};
use crate::tool::selection_tool::SelectedBrush;
use bevy::app::Plugin;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

pub struct CancelToolPlugin;

impl Plugin for CancelToolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, cancel_active_tool);
    }
}

fn cancel_active_tool(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
    selected_query: Query<Entity, With<SelectedBrush>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let is_grs = [
            EditorToolActive::Grabbing,
            EditorToolActive::Rotation,
            EditorToolActive::Scaling,
        ]
        .iter()
        .any(|t| *t == world_state.active_tool);
        if is_grs && !world_state.grs_tool_confirmed {
            for sb in &world_state.support_brushes {
                //look for brushes in selected brushes and substitute by sb manifolds - WIP
            }
        }
    }
}
