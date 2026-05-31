use bevy::prelude::*;

use crate::core::render::HoveredBrush;

#[derive(Component)]
pub struct SelectedBrush;

pub struct SelectToolPlugin;

impl Plugin for SelectToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, brush_selection_system);
    }
}

fn brush_selection_system(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    selected_query: Query<Entity, With<SelectedBrush>>,
    hovered_query: Query<Entity, With<HoveredBrush>>,
) {
    let mouse_lclick_pressed = mouse_input.just_pressed(MouseButton::Left);
    let keyboard_lshift_pressed = key_input.pressed(KeyCode::ShiftLeft);
    if let Some(hovered_entity) = hovered_query.iter().next() {
        if mouse_lclick_pressed && keyboard_lshift_pressed {
            if !selected_query.contains(hovered_entity) {
                commands.entity(hovered_entity).insert(SelectedBrush);
                commands.entity(hovered_entity).remove::<HoveredBrush>();
            } else {
                commands.entity(hovered_entity).remove::<SelectedBrush>();
            }
        } else if mouse_lclick_pressed {
            for entity in &selected_query {
                commands.entity(entity).remove::<SelectedBrush>();
            }
            commands.entity(hovered_entity).insert(SelectedBrush);
            commands.entity(hovered_entity).remove::<HoveredBrush>();
        }
    } else {
        if mouse_lclick_pressed {
            for entity in &selected_query {
                commands.entity(entity).remove::<SelectedBrush>();
            }
        }
    }
}
