use crate::core::world::EditorWorld;
use manifold_csg::MeshGL;

pub struct CsgCompiler;

impl CsgCompiler {
    pub fn compile_world(world: &EditorWorld) -> Option<MeshGL> {
        let world_manifold = world.base_brush.manifold.clone();

        let (vert_props, n_props, tri_indices) = world_manifold.to_mesh_f32();
        let mesh_gl = MeshGL::new(&vert_props, n_props, &tri_indices);
        Some(mesh_gl)
    }
}
