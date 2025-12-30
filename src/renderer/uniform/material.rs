use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy)]
pub struct MaterialData {
    pub base_color: [f32; 4],
    pub specular_color: [f32; 3],
    pub shininess: f32,
}
impl MaterialData {
    pub fn new(base_color: [f32; 4], specular_color: [f32; 3], shininess: f32) -> Self {
        Self {
            base_color,
            specular_color,
            shininess,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GpuMaterialData {
    base_color: [f32; 4],
    specular_color: [f32; 3],
    shininess: f32,
}

impl From<MaterialData> for GpuMaterialData {
    fn from(value: MaterialData) -> Self {
        Self {
            base_color: value.base_color,
            specular_color: value.specular_color,
            shininess: value.shininess,
        }
    }
}
