use crate::core::{
    brush::{self, Brush},
    geometry,
};
use bevy::prelude::*;
use manifold_csg::{Manifold, OpType};

#[derive(Debug, Clone)]
pub enum EditorToolActive {
    Add,
    Select,
    Transform,
}

#[derive(Debug, Clone)]
pub enum EditorMode {
    BakeMode,
    WorkshopMode,
}

#[derive(Resource, Debug, Clone)]
pub struct EditorWorld {
    pub active_tool: EditorToolActive,

    pub add_brush_mode: OpType,

    pub base_manifold: Manifold,

    pub brushes: Vec<Brush>,

    pub selected_brushes_indexes: Vec<usize>,

    pub highlighted_brush_index: Option<usize>,

    pub needs_rebuild: bool,

    pub grid_snap: f32,

    pub grid_height: f32,

    pub selected_brush_primitive: brush::BrushPrimitive,

    pub editor_mode: EditorMode,
}

impl Default for EditorWorld {
    fn default() -> Self {
        Self {
            active_tool: EditorToolActive::Add,
            add_brush_mode: OpType::Add,
            base_manifold: Manifold::empty()
                .transform(&geometry::bevy_transform_to_affine(Transform::IDENTITY)),
            brushes: Vec::new(),
            selected_brushes_indexes: Vec::new(),
            highlighted_brush_index: None,
            needs_rebuild: true,
            grid_snap: 1.0,
            grid_height: 0.0,
            selected_brush_primitive: brush::BrushPrimitive::Cube { size: Vec3::ONE },
            editor_mode: EditorMode::WorkshopMode,
        }
    }
}

impl EditorWorld {
    pub fn new() -> Self {
        Self {
            active_tool: EditorToolActive::Add,
            add_brush_mode: OpType::Add,
            base_manifold: Manifold::empty()
                .transform(&geometry::bevy_transform_to_affine(Transform::IDENTITY)),
            brushes: Vec::new(),
            selected_brushes_indexes: Vec::new(),
            highlighted_brush_index: None,
            needs_rebuild: true,
            grid_snap: 1.0,
            grid_height: 0.0,
            selected_brush_primitive: brush::BrushPrimitive::Cube { size: Vec3::ONE },
            editor_mode: EditorMode::WorkshopMode,
        }
    }

    pub fn apply_brush(&mut self, brush: &Brush) {
        self.base_manifold = self.base_manifold.boolean(&brush.manifold, brush.mode);
        self.needs_rebuild = true;
    }

    pub fn add_brush(&mut self, b: &Brush) {
        let aux_brush = b.clone();
        self.brushes.push(aux_brush);
    }

    pub fn remove_brush(&mut self, index: usize) {
        if index < self.brushes.len() {
            self.brushes.remove(index);

            self.selected_brushes_indexes.retain(|x| *x != index);
            self.needs_rebuild = true;
        }
    }

    pub fn swap_brush_order(&mut self, index_a: usize, index_b: usize) {
        if index_a < self.brushes.len() && index_b < self.brushes.len() {
            self.brushes.swap(index_a, index_b);
            self.needs_rebuild = true;
        }
    }

    pub fn bake(&mut self) {
        self.base_manifold = Manifold::empty();
        for br in self.brushes.clone() {
            self.apply_brush(&br);
        }

        self.needs_rebuild = true;
    }
}
