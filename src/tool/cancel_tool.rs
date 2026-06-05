use crate::core::world::{EditorToolActive, EditorWorld};
use crate::tool::selection_tool::SelectedBrush;
use bevy::app::Plugin;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use manifold_csg::MeshGL;

pub struct CancelToolPlugin;

impl Plugin for CancelToolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, cancel_active_tool);
    }
}

fn cancel_active_tool(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut world_state: ResMut<EditorWorld>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let is_grs = matches!(
            world_state.active_tool,
            EditorToolActive::Grabbing | EditorToolActive::Rotation | EditorToolActive::Scaling
        );
        if is_grs {
            let EditorWorld {
                brushes,
                support_brushes,
                grs_tool_confirmed,
                active_tool,
                needs_rebuild,
                ..
            } = &mut *world_state;
            for (brush_id, backup_manifold) in support_brushes.iter() {
                if let Some(br) = brushes.iter_mut().find(|b| &b.brush_entity_id == brush_id) {
                    br.manifold = backup_manifold.clone();

                    let (vert_props, n_props, tri_indices) = br.manifold.to_mesh_f32();
                    br.cached_meshgl = MeshGL::new(&vert_props, n_props, &tri_indices);
                }
            }
            support_brushes.clear();
            //*grs_tool_confirmed = false;
            *needs_rebuild = true;
            *active_tool = EditorToolActive::Selection;
        }
    }
}
