use bytemuck::{Pod, Zeroable};

pub mod frame;
pub mod light;
pub mod material;
pub mod model;

pub struct Uniform<T: Zeroable + Pod> {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Zeroable + Pod> Uniform<T> {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, visibility: &str) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(visibility),
            size: std::mem::size_of::<T>() as u64,
            //contents: bytemuck::cast_slice(&camera.view_projection_matrix().data.as_flattened()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(visibility),
            layout: layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn write(&self, queue: &wgpu::Queue, data: &T) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(data));
    }
}
