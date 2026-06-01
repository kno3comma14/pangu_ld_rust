// https://www.youtube.com/watch?v=mvCD7iaunVY
// https://www.youtube.com/watch?v=PtAxuOAQNcI
//
use crate::core::world::EditorToolActive;
use crate::core::world::EditorWorld;
use crate::tool::selection_tool::SelectedBrush;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GPosition(pub Vec3);

pub struct GrabToolPlugin;

impl Plugin for GrabToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GPosition(Vec3::ZERO))
            .add_systems(Update, grab_brush_system);
    }
}

fn grab_brush_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
    selected_query: Query<Entity, With<SelectedBrush>>,
    mut g_position: ResMut<GPosition>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        g_position.0 = world_state.central_mouse_coords;
        world_state.active_tool = EditorToolActive::Grabbing;
        if selected_query.is_empty() {
            return;
        } else {
            for b_idx in selected_query {}
        }
    }
}
