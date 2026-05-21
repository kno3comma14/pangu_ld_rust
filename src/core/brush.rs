use bevy::prelude::*;
use manifold_csg::Manifold;

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
        radius: f32,
        height: f32,
        segments: i32,
    },

    Cone {
        radius: f32,
        height: f32,
        segments: i32,
    },
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

    pub fn to_manifold(&self) -> Manifold {
        match self.primitive {
            BrushPrimitive::Cube { size } => {
                // manifold-csg: (x, y, z, center)
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
                radius,
                height,
                segments,
            } => Manifold::cylinder(height as f64, radius as f64, radius as f64, segments, true),
            BrushPrimitive::Cone {
                radius,
                height,
                segments,
            } => Manifold::cylinder(height as f64, radius as f64, 0.0, segments, true),
            BrushPrimitive::World => Manifold::empty(),
        }
    }
}
