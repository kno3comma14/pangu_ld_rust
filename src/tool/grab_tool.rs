use crate::core::world::EditorToolActive;
use crate::core::world::EditorWorld;
use crate::tool::selection_tool::SelectedBrush;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GPosition(pub Vec3);
#[derive(Resource)]
pub struct GrabModeActivated(pub bool);
#[derive(Resource)]
pub struct Confirmed(pub bool);

pub struct GrabToolPlugin;

impl Plugin for GrabToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GPosition(Vec3::ZERO))
            .insert_resource(GrabModeActivated(false))
            .insert_resource(Confirmed(false))
            .add_systems(Update, grab_brush_system);
    }
}

fn grab_brush_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
    selected_query: Query<Entity, With<SelectedBrush>>,
    mut g_position: ResMut<GPosition>,
    mut grab_mode_activated: ResMut<GrabModeActivated>,
    mut confirmed: ResMut<Confirmed>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if selected_query.is_empty() {
            return;
        }
        confirmed.0 = false;
        world_state.grs_tool_confirmed = false;
        g_position.0 = world_state.central_mouse_coords;
        world_state.active_tool = EditorToolActive::Grabbing;
    }

    grab_mode_activated.0 = world_state.active_tool == EditorToolActive::Grabbing;
    let space_pressed = keyboard_input.just_pressed(KeyCode::Space);

    if space_pressed && grab_mode_activated.0 {
        confirmed.0 = true;
        world_state.support_brushes.clear();
        world_state.grs_tool_confirmed = true;
        world_state.active_tool = EditorToolActive::Selection;
    }

    if grab_mode_activated.0 && !confirmed.0 {
        let total_mouse_delta = world_state.central_mouse_coords - g_position.0;
        for entity in &selected_query {
            let entity_id = entity.index_u32() as usize;
            if let Some(idx) = world_state
                .brushes
                .iter()
                .position(|b| b.brush_entity_id == entity_id)
            {
                let brush = &mut world_state.brushes[idx];
                let current_transform = Transform::from_translation(total_mouse_delta);
                brush.transform_brush(current_transform);
            }
        }
        g_position.0 = world_state.central_mouse_coords;
        world_state.needs_rebuild = true
    }
}
