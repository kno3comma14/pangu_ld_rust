use std::collections::VecDeque;

use crate::core::brush::Brush;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum EditorCommand {
    AddBrush {
        index: usize,
        brush: Brush,
    },
    DeleteBrush {
        index: usize,
        brush: Brush,
    },
    TransformBrushes {
        indices: Vec<usize>,
        from_transforms: Vec<Transform>,
        to_transforms: Vec<Transform>,
    },
    SwapBrushes {
        index_a: usize,
        index_b: usize,
    },
}

#[derive(Resource)]
pub struct EditorHistory {
    pub undo_stack: VecDeque<EditorCommand>,
    pub redo_stack: Vec<EditorCommand>,
    pub max_depth: usize,
}

impl Default for EditorHistory {
    fn default() -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(200),
            redo_stack: Vec::new(),
            max_depth: 200,
        }
    }
}

impl EditorHistory {
    // TODO
}

pub struct UndoRedoToolPlugin;

impl Plugin for UndoRedoToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, undo_redo_tool_system);
    }
}

pub fn undo_redo_tool_system() {}
