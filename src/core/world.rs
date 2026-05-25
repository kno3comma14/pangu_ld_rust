use crate::core::brush::{Brush, BrushMode, BrushPrimitive};
use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct EditorWorld {
    pub base_brush: Brush,

    pub brushes: Vec<Brush>,

    pub selected_brush_index: Option<usize>,

    pub needs_rebuild: bool,

    pub grid_snap: f32,

    pub grid_height: f32,
}

impl Default for EditorWorld {
    fn default() -> Self {
        Self {
            base_brush: Brush::new(
                BrushPrimitive::World,
                BrushMode::Additive,
                Transform::IDENTITY,
            ),
            brushes: Vec::new(),
            selected_brush_index: None,
            needs_rebuild: true,
            grid_snap: 1.0,
            grid_height: 0.0,
        }
    }
}

impl EditorWorld {
    pub fn new() -> Self {
        Self {
            base_brush: Brush::new(
                BrushPrimitive::World,
                BrushMode::Additive,
                Transform::IDENTITY,
            ),
            brushes: Vec::new(),
            selected_brush_index: None,
            needs_rebuild: true,
            grid_snap: 1.0,
            grid_height: 0.0,
        }
    }

    fn apply_brush(&mut self, brush: &Brush) {
        let mut m = brush.clone().manifold;
        let cols = brush.transform.to_matrix().to_cols_array();
        let t = [
            cols[0] as f64,
            cols[1] as f64,
            cols[2] as f64,
            cols[4] as f64, // row 0
            cols[5] as f64,
            cols[6] as f64,
            cols[8] as f64,
            cols[9] as f64, // row 1
            cols[10] as f64,
            cols[12] as f64,
            cols[13] as f64,
            cols[14] as f64, // row 2
        ];
        m = m.transform(&t);
        if brush.mode == BrushMode::Additive {
            self.base_brush.manifold = self.base_brush.manifold.union(&m);
        } else if brush.mode == BrushMode::Subtractive {
            self.base_brush.manifold = self.base_brush.manifold.difference(&m);
        } else {
            self.base_brush.manifold = self.base_brush.manifold.intersection(&m);
        }
    }

    pub fn add_brush(&mut self, brush: &Brush) {
        self.brushes.push(brush.clone());
        self.apply_brush(brush);
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
            //self.rebuild();
            self.needs_rebuild = true;
        }
    }

    pub fn swap_brush_order(&mut self, index_a: usize, index_b: usize) {
        if index_a < self.brushes.len() && index_b < self.brushes.len() {
            self.brushes.swap(index_a, index_b);
            self.rebuild();
            self.needs_rebuild = true;
        }
    }

    pub fn rebuild(&mut self) {
        self.base_brush = Brush::new(
            BrushPrimitive::World,
            BrushMode::Additive,
            self.base_brush.transform,
        );
        let brushes = self.brushes.clone();
        for b in brushes {
            self.apply_brush(&b);
        }
    }
}
