use crate::core::brush::Brush;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
pub struct EditorWorld {
    // pub base_brush: Brush,
    pub brushes: Vec<Brush>,

    pub selected_brush_index: Option<usize>,

    pub needs_rebuild: bool,

    pub grid_snap: f32,
}

impl EditorWorld {
    pub fn new() -> Self {
        Self {
            brushes: Vec::new(),
            selected_brush_index: None,
            needs_rebuild: true,
            grid_snap: 1.0,
        }
    }

    pub fn add_brush(&mut self, brush: Brush) {
        self.brushes.push(brush);
        self.needs_rebuild = true;
    }

    pub fn remove_brush(&mut self, index: usize) {
        if index < self.brushes.len() {
            self.brushes.remove(index);

            if let Some(selected) = self.selected_brush_index {
                if selected == index {
                    self.selected_brush_index = None;
                } else if selected > index {
                    self.selected_brush_index = Some(selected - 1);
                }
            }
            self.needs_rebuild = true;
        }
    }

    pub fn swap_brush_order(&mut self, index_a: usize, index_b: usize) {
        if index_a < self.brushes.len() && index_b < self.brushes.len() {
            self.brushes.swap(index_a, index_b);
            self.needs_rebuild = true;
        }
    }
}
