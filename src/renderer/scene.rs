use crate::renderer::manager::RenderManager;
use crate::renderer::model::importer::GltfFile;
use std::path::Path;

pub struct SceneInfo {
    pub vert_count: usize,
    pub index_count: usize,
    pub texture_count: usize,
    pub material_count: usize,
    pub mesh_count: usize,
    pub name: String,
}

/// Represents a loaded scene with all its meshes, materials, and textures
pub struct Scene {
    render_manager: RenderManager,
    path: String,
}

impl Scene {
    /// Load a new scene from a glTF file
    pub fn load(
        device: &wgpu::Device,
        material_layout: &wgpu::BindGroupLayout,
        queue: &wgpu::Queue,
        path: &Path,
    ) -> Result<Self, String> {
        let file =
            GltfFile::load_gltf(path).map_err(|e| format!("Failed to load glTF file: {:?}", e))?;

        let render_manager = RenderManager::new(device, material_layout, queue, &file);

        Ok(Self {
            render_manager,
            path: path.to_string_lossy().to_string(),
        })
    }

    pub fn render_manager(&self) -> &RenderManager {
        &self.render_manager
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn render(&self, renderpass: &mut wgpu::RenderPass) {
        self.render_manager.render(renderpass);
    }

    pub fn get_info(&self) -> SceneInfo {
        SceneInfo {
            vert_count: self.render_manager.vertex_count(),
            index_count: self.render_manager.index_count(),
            texture_count: self.render_manager.texture_count(),
            material_count: self.render_manager.material_count(),
            mesh_count: self.render_manager.mesh_count(),
            name: self.path.clone(),
        }
    }
}
