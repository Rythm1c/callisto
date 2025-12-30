use bytemuck::{Pod, Zeroable};
use math::mat4::Mat4;

#[derive(Debug, Clone, Copy)]
pub struct ModelData {
    pub world: Mat4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct GpuModelData {
    pub world: [f32; 16],
}

impl From<ModelData> for GpuModelData {
    fn from(value: ModelData) -> Self {
        Self {
            world: value.world.flattended(),
        }
    }
}
