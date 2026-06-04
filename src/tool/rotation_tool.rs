use crate::core::world::EditorToolActive;
use crate::core::world::EditorWorld;
use crate::tool::selection_tool::SelectedBrush;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

#[derive(Resource)]
pub struct RefRotation(pub Quat);
#[derive(Resource)]
pub struct Confirmed(pub bool);
#[derive(Resource)]
pub struct PivotePosition(pub Vec2);

pub struct RotationToolPlugin;

impl Plugin for RotationToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Confirmed(false))
            .insert_resource(RefRotation(Quat::IDENTITY))
            .insert_resource(PivotePosition(Vec2::ZERO))
            .add_systems(Update, rotation_brush_system);
    }
}

fn rotation_brush_system(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
    selected_query: Query<Entity, With<SelectedBrush>>,
    mut confirmed: ResMut<Confirmed>,
    mut reference_rotation: ResMut<RefRotation>,
    mut pivote_position: ResMut<PivotePosition>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if selected_query.is_empty() {
            return;
        }
        confirmed.0 = false;
        reference_rotation.0 = Quat::IDENTITY;
        pivote_position.0 = cursor_position;
        world_state.active_tool = EditorToolActive::Rotation;
    }

    let space_pressed = keyboard_input.just_pressed(KeyCode::Space);

    if space_pressed && world_state.active_tool == EditorToolActive::Rotation {
        confirmed.0 = true;
        world_state.support_brushes.clear();
        world_state.active_tool = EditorToolActive::Selection;
    }

    if world_state.active_tool == EditorToolActive::Rotation && !confirmed.0 {
        let total_mouse_delta = cursor_position - pivote_position.0;
        let y_rotation = Quat::from_rotation_y(total_mouse_delta.y * 0.05);

        for entity in &selected_query {
            let entity_id = entity.index_u32() as usize;
            if let Some(idx) = world_state
                .brushes
                .iter()
                .position(|b| b.brush_entity_id == entity_id)
            {
                let brush = &mut world_state.brushes[idx];
                let pivot = brush.calculate_center();
                let delta_transform = Transform::from_translation(pivot)
                    * Transform::from_rotation(y_rotation)
                    * Transform::from_translation(-pivot);
                brush.transform_brush_relative(delta_transform, y_rotation);
            }
        }
        pivote_position.0 = cursor_position;
        world_state.needs_rebuild = true;
    }
}
