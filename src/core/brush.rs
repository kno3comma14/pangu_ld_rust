use bevy::prelude::*;
use manifold_csg::{CrossSection, Manifold, MeshGL, OpType};

use crate::core::geometry;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrushPrimitive {
    Cube {
        size: Vec3,
    },

    Sphere {
        radius: f32,
        subdivisions: i32,
    },

    Cylinder {
        height: f32,
        radius_low: f32,
        radius_high: f32,
        segments: i32,
    },

    Cone {
        radius: f32,
        height: f32,
        segments: i32,
    },

    Torus {
        major_radius: f32,
        minor_radius: f32,
        segments: i32,
    },

    Tetrahedron,

    World,
    Empty,
}

#[derive(Component, Clone)]
pub struct Brush {
    pub base_primitive: BrushPrimitive,
    pub manifold: Manifold,
    pub cached_meshgl: MeshGL,
    pub mode: OpType,
    pub transform: Transform,
    pub brush_entity_id: usize,
}

impl std::fmt::Debug for Brush {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Brush")
            .field("base_primitive", &self.base_primitive)
            // Manifold also doesn't implement Debug, so print a placeholder
            .field("manifold", &"<Manifold Opaque Handle>")
            // Print structural stats for MeshGL instead of raw data
            .field(
                "cached_meshgl",
                &format!("MeshGL (num_triangles: {})", self.cached_meshgl.num_tri()),
            )
            .finish()
    }
}

impl Brush {
    pub fn new(primitive: BrushPrimitive, mode: OpType, transform: Transform) -> Brush {
        let mut m = match primitive {
            BrushPrimitive::Cube { size } => {
                Manifold::cube(size.x as f64, size.y as f64, size.z as f64, true)
            }
            BrushPrimitive::Sphere {
                radius,
                subdivisions,
            } => Manifold::sphere(radius as f64, subdivisions),
            BrushPrimitive::Cylinder {
                height,
                radius_low,
                radius_high,
                segments,
            } => Manifold::cylinder(
                height as f64,
                radius_low as f64,
                radius_high as f64,
                segments,
                true,
            ),
            BrushPrimitive::Cone {
                radius,
                height,
                segments,
            } => Manifold::cylinder(height as f64, radius as f64, 0.0, segments, true),
            BrushPrimitive::Torus {
                major_radius,
                minor_radius,
                segments,
            } => {
                let circle = CrossSection::circle((major_radius - minor_radius) as f64, segments);
                let offset_circle = circle.translate(major_radius as f64, 0.0);
                let torus = Manifold::revolve(&offset_circle, segments, 360.0);
                torus
            }
            BrushPrimitive::World => Manifold::empty(),
            BrushPrimitive::Empty => Manifold::empty(),
            BrushPrimitive::Tetrahedron => Manifold::tetrahedron(),
            _ => Manifold::empty(),
        };
        let affine = geometry::bevy_transform_to_affine(transform);
        m = m.transform(&affine);
        let (vert_props, n_props, tri_indices) = m.to_mesh_f32();
        Self {
            base_primitive: primitive,
            manifold: m,
            cached_meshgl: MeshGL::new(&vert_props, n_props, &tri_indices),
            mode: mode,
            transform: transform,
            brush_entity_id: 0,
        }
    }

    pub fn transform_brush(&mut self, t: Transform) {
        let affine = geometry::bevy_transform_to_affine(t);
        self.transform = t;
        self.manifold = self.manifold.transform(&affine);
        let (vert_props, n_props, tri_indices) = self.manifold.to_mesh_f32();
        self.cached_meshgl = MeshGL::new(&vert_props, n_props, &tri_indices);
    }

    pub fn generate_preview_transform(&mut self, t: Transform) {
        let mut future_mesh = self.clone();
    }
}
