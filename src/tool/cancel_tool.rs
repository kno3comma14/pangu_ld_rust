use crate::core::world::EditorWorld;
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
    if keyboard_input.just_pressed(KeyCode::Escape) {}
}
