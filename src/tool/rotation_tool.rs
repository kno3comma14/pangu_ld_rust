use crate::core::world::EditorToolActive;
use crate::core::world::EditorWorld;
use crate::tool::axis_lock::AxisLockType;
use crate::tool::axis_lock::{TransformType, restrict_transform};
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
#[derive(Resource)]
pub struct AxisLock(pub AxisLockType);

pub struct RotationToolPlugin;

impl Plugin for RotationToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Confirmed(false))
            .insert_resource(AxisLock(AxisLockType::Global))
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
    mut al: ResMut<AxisLock>,
) {
    let transform_type = TransformType::Rotation;
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
        world_state.grs_tool_confirmed = false;
        reference_rotation.0 = Quat::IDENTITY;
        pivote_position.0 = cursor_position;
        al.0 = AxisLockType::Global;
        world_state.active_tool = EditorToolActive::Rotation;
    }

    let space_pressed = keyboard_input.just_pressed(KeyCode::Space);

    if space_pressed && world_state.active_tool == EditorToolActive::Rotation {
        confirmed.0 = true;
        al.0 = AxisLockType::Global;
        world_state.support_brushes.clear();
        for entity in selected_query {
            let idx = entity.index_u32() as usize;
            if let Some(mani) = world_state
                .brushes
                .iter()
                .find(|br| br.brush_entity_id == idx)
                .map(|item| item.manifold.clone())
            {
                world_state.support_brushes.insert(idx, mani);
            }
        }
        world_state.grs_tool_confirmed = true;
        world_state.active_tool = EditorToolActive::Selection;
    }

    if world_state.active_tool == EditorToolActive::Rotation && !confirmed.0 {
        // 1. Persistently update lock state based on user key presses
        if keyboard_input.just_pressed(KeyCode::KeyX) {
            al.0 = AxisLockType::X;
        } else if keyboard_input.just_pressed(KeyCode::KeyY) {
            al.0 = AxisLockType::Y;
        } else if keyboard_input.just_pressed(KeyCode::KeyZ) {
            al.0 = AxisLockType::Z;
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            al.0 = AxisLockType::Global;
        }

        let mouse_delta = cursor_position - pivote_position.0;
        let rotation_amount = mouse_delta.y * 0.05;

        if rotation_amount.abs() > 0.0001 {
            for entity in &selected_query {
                let entity_id = entity.index_u32() as usize;
                if let Some(idx) = world_state
                    .brushes
                    .iter()
                    .position(|b| b.brush_entity_id == entity_id)
                {
                    let brush = &mut world_state.brushes[idx];
                    let pivot = brush.calculate_center();

                    let axis_vector = match al.0 {
                        AxisLockType::X => Vec3::X,
                        AxisLockType::Y => Vec3::Y,
                        AxisLockType::Z => Vec3::Z,
                        AxisLockType::Global => Vec3::Y,
                    };
                    let local_axis = brush.transform.rotation * axis_vector;

                    let calculated_rotation = Quat::from_axis_angle(local_axis, rotation_amount);

                    let delta_transform = Transform::from_translation(pivot)
                        * Transform::from_rotation(calculated_rotation)
                        * Transform::from_translation(-pivot);
                    let restricted_transform =
                        restrict_transform(delta_transform, al.0, transform_type);
                    brush.transform_brush_relative(restricted_transform, calculated_rotation);
                }
            }
            world_state.needs_rebuild = true;
        }
        pivote_position.0 = cursor_position;
    }
}
