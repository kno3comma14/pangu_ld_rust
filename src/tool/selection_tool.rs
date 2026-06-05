use bevy::prelude::*;

use crate::core::render::HoveredBrush;
use crate::core::render::WorkshopBrushMarker;
use crate::core::world::EditorWorld;

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
    mut world_state: ResMut<EditorWorld>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    selected_query: Query<Entity, With<SelectedBrush>>,
    hovered_query: Query<Entity, With<HoveredBrush>>,
    workshop_mesh_query: Query<(Entity, &WorkshopBrushMarker)>,
) {
    // let in_select_mode = world_state.active_tool == EditorToolActive::Selection; For future validation
    let mouse_lclick_pressed = mouse_input.just_pressed(MouseButton::Left);
    let keyboard_lshift_pressed = key_input.pressed(KeyCode::ShiftLeft);

    if let Some(hovered_entity) = hovered_query.iter().next() {
        if mouse_lclick_pressed && keyboard_lshift_pressed {
            if !selected_query.contains(hovered_entity) {
                let idx = hovered_entity.index_u32() as usize;
                world_state.selected_brushes_indexes.push(idx);
                commands.entity(hovered_entity).insert(SelectedBrush);
                if let Some(bid) = workshop_mesh_query
                    .iter()
                    .find(|wmi| wmi.0.index_u32() as usize == idx)
                    .map(|item| item.1.brush_id)
                {
                    if !world_state.support_brushes.contains_key(&idx) {
                        let initial_manifold = world_state.brushes[bid].manifold.clone();
                        world_state.support_brushes.insert(idx, initial_manifold);
                    }
                }
                commands.entity(hovered_entity).remove::<HoveredBrush>();
            } else {
                commands.entity(hovered_entity).remove::<SelectedBrush>();
                if let Some(idx) = world_state
                    .selected_brushes_indexes
                    .iter()
                    .position(|&x| x == hovered_entity.index_u32() as usize)
                {
                    _ = world_state.selected_brushes_indexes.remove(idx);
                    world_state.support_brushes.remove(&idx);
                }
            }
        } else if mouse_lclick_pressed {
            for entity in &selected_query {
                commands.entity(entity).remove::<SelectedBrush>();
            }
            world_state.selected_brushes_indexes.clear();
            world_state.support_brushes.clear();

            let idx = hovered_entity.index_u32() as usize;
            commands.entity(hovered_entity).insert(SelectedBrush);
            world_state.selected_brushes_indexes.push(idx);
            if !world_state.support_brushes.contains_key(&idx) {
                let initial_manifold = world_state
                    .brushes
                    .iter()
                    .find(|&b| b.brush_entity_id == idx)
                    .map(|target_brush| target_brush.manifold.clone());
                if let Some(manifold) = initial_manifold {
                    world_state.support_brushes.insert(idx, manifold);
                }
            }

            commands.entity(hovered_entity).remove::<HoveredBrush>();
        }
    } else {
        if mouse_lclick_pressed {
            for entity in &selected_query {
                commands.entity(entity).remove::<SelectedBrush>();
            }
            world_state.selected_brushes_indexes.clear();
            world_state.support_brushes.clear();
        }
    }
}
