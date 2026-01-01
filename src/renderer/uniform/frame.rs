use bytemuck::{Pod, Zeroable};
use math::{mat4::Mat4, vec3::Vec3};

use crate::renderer::light::DirectionalLight;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct GpuDirectionalLight {
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

#[derive(Clone, Copy)]
pub struct FrameData {
    pub view_proj: Mat4,
    pub cam_pos: Vec3,
    pub sun: DirectionalLight,
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct GpuFrameData {
    view_proj: [f32; 16],

    cam_pos: [f32; 3],
    _pad0: f32, //padding

    sun: GpuDirectionalLight,
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

pub struct FrameBindGroup {
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl FrameBindGroup {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: std::mem::size_of::<GpuFrameData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }

    pub fn write(&self, queue: &wgpu::Queue, data: FrameData) {
        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::bytes_of(&GpuFrameData::from(data)),
        );
    }
}
