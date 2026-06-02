use crate::core::world;
// https://www.youtube.com/watch?v=mvCD7iaunVY
// https://www.youtube.com/watch?v=PtAxuOAQNcI
//
use crate::core::world::EditorToolActive;
use crate::core::world::EditorWorld;
use crate::tool::selection_tool::SelectedBrush;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GPosition(pub Vec3);

#[derive(Resource)]
pub struct GrabModeActivated(pub bool);

pub struct GrabToolPlugin;

impl Plugin for GrabToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GPosition(Vec3::ZERO))
            .insert_resource(GrabModeActivated(false))
            .add_systems(Update, grab_brush_system);
    }
}

fn grab_brush_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
    selected_query: Query<Entity, With<SelectedBrush>>,
    mut g_position: ResMut<GPosition>,
    mut grab_mode_activated: ResMut<GrabModeActivated>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if selected_query.is_empty() {
            return;
        }

        g_position.0 = world_state.central_mouse_coords;
        world_state.active_tool = EditorToolActive::Grabbing;
        grab_mode_activated.0 = true;
    }

    if grab_mode_activated.0 {
        let total_mouse_delta = world_state.central_mouse_coords - g_position.0;
        for entity in &selected_query {
            let entity_id = entity.index_u32() as usize;
            if let Some(idx) = world_state
                .brushes
                .iter()
                .position(|b| b.brush_entity_id == entity_id)
            {
                let brush = &mut world_state.brushes[idx];
                brush.transform = Transform::from_translation(total_mouse_delta);
                let current_transform = brush.transform;
                brush.transform_brush(current_transform);
            }
        }
        g_position.0 = world_state.central_mouse_coords;
        world_state.bake();
    }
}
