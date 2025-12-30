use bytemuck::{Pod, Zeroable};

use crate::renderer::light::DirectionalLight;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GpuDirectionalLight {
    direction: [f32; 3],
    _pad0: f32, //padding

    color: [f32; 3],
    _pad1: f32, //padding
}

impl From<DirectionalLight> for GpuDirectionalLight {
    fn from(value: DirectionalLight) -> Self {
        Self {
            direction: value.direction.to_array(),
            _pad0: 0.0,
            color: value.color,
            _pad1: 0.0,
        }
    }
}
