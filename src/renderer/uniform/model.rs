use bytemuck::{Pod, Zeroable};
use math::mat4::Mat4;

#[derive(Debug, Clone, Copy)]
pub struct ModelData {
    pub world: Mat4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
struct GpuModelData {
    world: [f32; 16],
}

impl From<ModelData> for GpuModelData {
    fn from(value: ModelData) -> Self {
        Self {
            world: value.world.flattended(),
        }
    }
}

pub struct ModelBindGroup {
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl ModelBindGroup {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: std::mem::size_of::<GpuModelData>() as u64,
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

    pub fn write(&self, queue: &wgpu::Queue, data: ModelData) {
        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::bytes_of(&GpuModelData::from(data)),
        );
    }
}
