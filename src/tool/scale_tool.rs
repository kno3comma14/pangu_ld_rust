use crate::core::world::EditorToolActive;
use crate::core::world::EditorWorld;
use crate::tool::selection_tool::SelectedBrush;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

#[derive(Resource)]
pub struct RefScale(pub Vec3);
#[derive(Resource)]
pub struct Confirmed(pub bool);
#[derive(Resource)]
pub struct PivotePosition(pub Vec2);

pub struct ScaleToolPlugin;

impl Plugin for ScaleToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Confirmed(false))
            .insert_resource(RefScale(Vec3::ONE))
            .insert_resource(PivotePosition(Vec2::ZERO))
            .add_systems(Update, scale_brush_system);
    }
}

fn scale_brush_system(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
    selected_query: Query<Entity, With<SelectedBrush>>,
    mut confirmed: ResMut<Confirmed>,
    mut reference_scale: ResMut<RefScale>,
    mut pivote_position: ResMut<PivotePosition>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::KeyT) {
        if selected_query.is_empty() {
            return;
        }
        confirmed.0 = false;
        world_state.grs_tool_confirmed = false;
        reference_scale.0 = Vec3::ONE;
        pivote_position.0 = cursor_position;
        world_state.active_tool = EditorToolActive::Scaling;
    }

    let space_pressed = keyboard_input.just_pressed(KeyCode::Space);

    if space_pressed && world_state.active_tool == EditorToolActive::Scaling {
        confirmed.0 = true;
        world_state.grs_tool_confirmed = true;
        world_state.support_brushes.clear();
        world_state.active_tool = EditorToolActive::Selection;
    }

    if world_state.active_tool == EditorToolActive::Scaling && !confirmed.0 {
        let total_mouse_delta = cursor_position - pivote_position.0;
        let scale_factor = (1.0 - total_mouse_delta.y * 0.01).max(0.001);
        let delta_scale = Vec3::splat(scale_factor);

        for entity in &selected_query {
            let entity_id = entity.index_u32() as usize;
            if let Some(idx) = world_state
                .brushes
                .iter()
                .position(|b| b.brush_entity_id == entity_id)
            {
                let brush = &mut world_state.brushes[idx];
                let pivot = brush.calculate_center();
                let current_transform = Transform::from_translation(pivot)
                    * Transform::from_scale(delta_scale)
                    * Transform::from_translation(-pivot);
                brush.transform_brush(current_transform);
            }
        }
        pivote_position.0 = cursor_position;
        world_state.needs_rebuild = true;
    }
}
