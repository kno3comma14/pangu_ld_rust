use bevy::prelude::*;
use manifold_csg::{CrossSection, Manifold};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BrushMode {
    #[default]
    Additive,
    Subtractive,
    Intersect,
}

#[derive(Component, Debug, Clone)]
pub struct Brush {
    pub primitive: BrushPrimitive,

    pub mode: BrushMode,

    pub transform: Transform,
}

impl Brush {
    pub fn new_cube(size: Vec3, mode: BrushMode, transform: Transform) -> Self {
        Self {
            primitive: BrushPrimitive::Cube { size },
            mode,
            transform,
        }
    }

    pub fn new_sphere(
        radius: f32,
        subdivisions: i32,
        mode: BrushMode,
        transform: Transform,
    ) -> Self {
        Self {
            primitive: BrushPrimitive::Sphere {
                radius,
                subdivisions,
            },
            mode,
            transform,
        }
    }

    pub fn new_world() -> Self {
        Self {
            primitive: BrushPrimitive::World {},
            mode: BrushMode::Additive,
            transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)),
        }
    }

    pub fn new_cylinder(
        height: f32,
        radius_low: f32,
        radius_high: f32,
        segments: i32,
        mode: BrushMode,
        transform: Transform,
    ) -> Self {
        Self {
            primitive: BrushPrimitive::Cylinder {
                height,
                radius_low,
                radius_high,
                segments,
            },
            mode,
            transform,
        }
    }

    pub fn new_torus(
        major_radius: f32,
        minor_radius: f32,
        segments: i32,
        mode: BrushMode,
        transform: Transform,
    ) -> Self {
        Self {
            primitive: BrushPrimitive::Torus {
                major_radius,
                minor_radius,
                segments,
            },
            mode,
            transform,
        }
    }

    pub fn to_manifold(&self) -> Manifold {
        match self.primitive {
            BrushPrimitive::Cube { size } => {
                Manifold::cube(size.x as f64, size.y as f64, size.z as f64, true)
            }
            BrushPrimitive::Sphere {
                radius,
                subdivisions,
            } => {
                // manifold-csg: (radius, segments)
                Manifold::sphere(radius as f64, subdivisions)
            }
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
                let circle = CrossSection::circle(minor_radius as f64, segments);
                let offset_circle = circle.translate(major_radius as f64, 0.0);
                let torus = Manifold::revolve(&offset_circle, segments, 360.0);
                torus
            }
            BrushPrimitive::World => Manifold::empty(),
            BrushPrimitive::Tetrahedron => Manifold::tetrahedron(),
        }
    }
}
