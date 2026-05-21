use crate::core::brush::BrushMode;
use crate::core::world::EditorWorld;
use manifold_csg::{Manifold, MeshGL};

pub struct CsgCompiler;

impl CsgCompiler {
    pub fn compile_world(world: &EditorWorld) -> Option<MeshGL> {
        if world.brushes.is_empty() {
            return None;
        }

        let mut world_manifold: Option<Manifold> = None;

        for brush in &world.brushes {
            let mut brush_manifold = brush.to_manifold();

            let (scale, rotation, translation) = (
                brush.transform.scale,
                brush.transform.rotation,
                brush.transform.translation,
            );

            brush_manifold = brush_manifold.scale(scale.x as f64, scale.y as f64, scale.z as f64);
            brush_manifold = brush_manifold.translate(
                translation.x as f64,
                translation.y as f64,
                translation.z as f64,
            );
            brush_manifold =
                brush_manifold.rotate(rotation.x as f64, rotation.y as f64, rotation.z as f64);

            match world_manifold {
                None => {
                    world_manifold = Some(brush_manifold);
                }
                Some(current_world) => {
                    let updated_world = match brush.mode {
                        BrushMode::Additive => current_world.union(&brush_manifold),
                        BrushMode::Subtractive => current_world.difference(&brush_manifold),
                        BrushMode::Intersect => current_world.intersection(&brush_manifold),
                    };
                    world_manifold = Some(updated_world);
                }
            }
        }

        world_manifold.map(|final_manifold| {
            let (vert_props, n_props, tri_indices) = final_manifold.to_mesh_f32();
            let mesh_gl = MeshGL::new(&vert_props, n_props, &tri_indices);
            mesh_gl
        })
    }
}
