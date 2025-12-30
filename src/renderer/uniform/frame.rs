use bytemuck::{Pod, Zeroable};
use math::{mat4::Mat4, vec3::Vec3};

use crate::renderer::{light::DirectionalLight, uniform::light::GpuDirectionalLight};

#[derive(Clone, Copy)]
pub struct FrameData {
    pub view_proj: Mat4,
    pub cam_pos: Vec3,
    pub sun: DirectionalLight,
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GpuFrameData {
    pub view_proj: [f32; 16],

    pub cam_pos: [f32; 3],
    _pad0: f32, //padding

    pub sun: GpuDirectionalLight,
}

impl From<FrameData> for GpuFrameData {
    fn from(value: FrameData) -> Self {
        Self {
            view_proj: value.view_proj.flattended(),
            cam_pos: value.cam_pos.to_array(),
            _pad0: 0.0,
            sun: value.sun.into(),
        }
    }
}

