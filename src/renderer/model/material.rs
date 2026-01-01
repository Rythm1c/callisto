#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub ao: f32,
    pub base_color_factor: [f32; 4],
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    // textures
    pub base_color_texture: Option<usize>,
    pub metallic_roughness_texture: Option<usize>,
    pub normal_texture: Option<usize>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ao: 1.0,
            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            metallic_factor: 0.5,
            roughness_factor: 0.5,
            base_color_texture: None,
            metallic_roughness_texture: None,
            normal_texture: None,
        }
    }
}

impl Material {
    pub fn from_gltf(material: &gltf::Material) -> Self {
        let pbr = &material.pbr_metallic_roughness();

        let mut base_color_texture: Option<usize> = None;
        if let Some(base_tex) = pbr.base_color_texture() {
            base_color_texture = Some(base_tex.texture().index());
        }

        let mut metallic_roughness_texture: Option<usize> = None;
        if let Some(metallic_tex) = pbr.metallic_roughness_texture() {
            metallic_roughness_texture = Some(metallic_tex.texture().index());
        }

        let mut normal_texture: Option<usize> = None;
        if let Some(normal_tex) = material.normal_texture() {
            normal_texture = Some(normal_tex.texture().index());
        }

        Self {
            ao: 1.0,
            base_color_factor: pbr.base_color_factor(),
            metallic_factor: pbr.metallic_factor(),
            roughness_factor: pbr.roughness_factor(),
            base_color_texture,
            metallic_roughness_texture,
            normal_texture,
        }
    }

    pub fn get_texture_flags(&self) -> TextureFlags {
        TextureFlags {
            has_base_color: self.base_color_texture.is_some() as u32,
            has_metallic_roughness: self.metallic_roughness_texture.is_some() as u32,
            has_normal: self.normal_texture.is_some() as u32,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TextureFlags {
    pub has_base_color: u32,
    pub has_metallic_roughness: u32,
    pub has_normal: u32,
}
