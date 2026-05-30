use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use manifold_csg::OpType;

use crate::core::{brush::BrushPrimitive, world::EditorWorld};

pub fn primitive_selector_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
) {
    if keyboard.just_pressed(KeyCode::Digit4) {
        world_state.selected_brush_primitive = BrushPrimitive::Cube { size: Vec3::ONE };
        println!("Selected Tool: Cube");
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        world_state.selected_brush_primitive = BrushPrimitive::Sphere {
            radius: 0.5,
            subdivisions: 64,
        };
        println!("Selected Tool: Sphere");
    } else if keyboard.just_pressed(KeyCode::Digit6) {
        world_state.selected_brush_primitive = BrushPrimitive::Cylinder {
            height: 2.0,
            radius_low: 0.5,
            radius_high: 0.5,
            segments: 16,
        };
        println!("Selected Tool: Cylinder");
    } else if keyboard.just_pressed(KeyCode::Digit7) {
        world_state.selected_brush_primitive = BrushPrimitive::Cone {
            radius: 0.5,
            height: 1.5,
            segments: 16,
        };
        println!("Selected Tool: Cone");
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        if world_state.add_brush_mode == OpType::Add {
            world_state.add_brush_mode = OpType::Subtract;
        } else {
            world_state.add_brush_mode = OpType::Add;
        }
    }
}
