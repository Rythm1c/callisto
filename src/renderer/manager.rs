use crate::renderer::model::importer::GltfFile;
use crate::renderer::model::material::Material;
use crate::renderer::model::mesh::Mesh;
use crate::renderer::model::texture::Texture;
use crate::renderer::traits::Renderable;
use crate::renderer::uniform::material::PbrMaterial;

/// Manages fallback textures used when materials don't have specific texture maps
#[derive(Clone)]
struct FallbackTextures {
    base_color: Texture,
    metallic_roughness: Texture,
    normal: Texture,
}

impl FallbackTextures {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self {
            base_color: Texture::fallback_white(device, queue),
            metallic_roughness: Texture::fallback_metallic_roughness(device, queue),
            normal: Texture::fallback_normal(device, queue),
        }
    }
}

#[derive(Clone)]
pub struct RenderManager {
    meshes: Vec<Mesh>,
    textures: Vec<Texture>,
    materials: Vec<PbrMaterial>,
    default_material: PbrMaterial,
    fallbacks: FallbackTextures,
}

impl RenderManager {
    pub fn new(
        device: &wgpu::Device,
        material_layout: &wgpu::BindGroupLayout,
        queue: &wgpu::Queue,
        file: &GltfFile,
    ) -> Self {
        let meshes = file
            .get_document()
            .meshes()
            .map(|mesh| Mesh::from_gltf(device, &mesh, &file))
            .collect();

        let textures: Vec<Texture> = file
            .get_document()
            .textures()
            .map(|texture| Texture::from_gltf(device, queue, &texture, &file, None))
            .collect();

        let fallbacks = FallbackTextures::new(device, queue);

        let cpu_materials: Vec<Material> = file
            .get_document()
            .materials()
            .map(|material| Material::from_gltf(&material))
            .collect();

        let materials = cpu_materials
            .iter()
            .map(|material| {
                Self::create_pbr_material(device, material_layout, material, &textures, &fallbacks)
            })
            .collect();

        let default_material = Self::create_pbr_material(
            device,
            material_layout,
            &Material::default(),
            &textures,
            &fallbacks,
        );

        Self {
            meshes,
            textures,
            materials,
            default_material,
            fallbacks,
        }
    }

    /// Create a PbrMaterial from a CPU material, using textures or fallbacks
    fn create_pbr_material(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        material: &Material,
        textures: &[Texture],
        fallbacks: &FallbackTextures,
    ) -> PbrMaterial {
        let base_tex = material
            .base_color_texture
            .map(|idx| &textures[idx])
            .unwrap_or(&fallbacks.base_color);

        let mr_tex = material
            .metallic_roughness_texture
            .map(|idx| &textures[idx])
            .unwrap_or(&fallbacks.metallic_roughness);

        let normal_tex = material
            .normal_texture
            .map(|idx| &textures[idx])
            .unwrap_or(&fallbacks.normal);

        PbrMaterial::new(
            device,
            layout,
            material,
            (base_tex.get_view(), base_tex.get_sampler()),
            (mr_tex.get_view(), mr_tex.get_sampler()),
            (normal_tex.get_view(), normal_tex.get_sampler()),
        )
    }

    /// Render all meshes and their primitives with their respective materials
    pub fn render(&self, renderpass: &mut wgpu::RenderPass) {
        self.meshes.iter().for_each(|mesh| {
            mesh.get_primitives().iter().for_each(|primitive| {
                let pbr_material = primitive
                    .get_material()
                    .and_then(|idx| self.materials.get(idx))
                    .unwrap_or(&self.default_material);

                renderpass.set_bind_group(2, &pbr_material.bind_group, &[]);
                primitive.render(renderpass);
            });
        });
    }

    // ==================== Accessors ====================
    pub fn get_material(&self, index: usize) -> Option<&PbrMaterial> {
        self.materials.get(index)
    }

    pub fn get_texture(&self, index: usize) -> Option<&Texture> {
        self.textures.get(index)
    }

    pub fn material_count(&self) -> usize {
        self.materials.len()
    }

    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }

    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    pub fn vertex_count(&self) -> usize {
        self.meshes.iter().map(|mesh| mesh.get_vert_count()).sum()
    }

    pub fn index_count(&self) -> usize {
        self.meshes.iter().map(|mesh| mesh.get_index_count()).sum()
    }
}
