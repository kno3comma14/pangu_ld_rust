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

    _Torus {
        major_radius: f32,
        minor_radius: f32,
        segments: i32,
    },

    _Tetrahedron,

    _World,
    _Empty,
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
            .field("manifold", &"<Manifold Opaque Handle>")
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
            BrushPrimitive::_Torus {
                major_radius,
                minor_radius,
                segments,
            } => {
                let circle = CrossSection::circle((major_radius - minor_radius) as f64, segments);
                let offset_circle = circle.translate(major_radius as f64, 0.0);
                let torus = Manifold::revolve(&offset_circle, segments, 360.0);
                torus
            }
            BrushPrimitive::_World => Manifold::empty(),
            BrushPrimitive::_Empty => Manifold::empty(),
            BrushPrimitive::_Tetrahedron => Manifold::tetrahedron(),
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

    pub fn calculate_center(&self) -> Vec3 {
        let (vert_props, _, _) = self.manifold.to_mesh_f32();
        if vert_props.is_empty() {
            return Vec3::ZERO;
        }

        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        for chunk in vert_props.chunks_exact(3) {
            let pos = Vec3::new(chunk[0], chunk[1], chunk[2]);
            min = min.min(pos);
            max = max.max(pos);
        }
        (min + max) * 0.5
    }

    pub fn transform_brush(&mut self, t: Transform) {
        let affine = geometry::bevy_transform_to_affine(t);
        self.transform = t;
        self.manifold = self.manifold.transform(&affine);
        let (vert_props, n_props, tri_indices) = self.manifold.to_mesh_f32();
        self.cached_meshgl = MeshGL::new(&vert_props, n_props, &tri_indices);
    }

    pub fn transform_brush_relative(&mut self, relative_t: Transform, rotation_delta: Quat) {
        let affine = geometry::bevy_transform_to_affine(relative_t);
        self.manifold = self.manifold.transform(&affine);
        self.transform.rotation = rotation_delta * self.transform.rotation;
        let (vert_props, n_props, tri_indices) = self.manifold.to_mesh_f32();
        self.cached_meshgl = MeshGL::new(&vert_props, n_props, &tri_indices);
    }
}
