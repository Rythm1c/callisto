use crate::renderer::model::material::Material;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod, Debug, Default)]
pub struct GpuMaterial {
    pub base_color_factor: [f32; 4],
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub ao: f32,
    _pad: [f32; 4], // 16-byte alignment
}

impl From<&Material> for GpuMaterial {
    fn from(mat: &Material) -> Self {
        Self {
            base_color_factor: mat.base_color_factor,
            metallic_factor: mat.metallic_factor,
            roughness_factor: mat.roughness_factor,
            ao: mat.ao,
            _pad: [0.0; 4],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod, Debug, Default)]
struct GpuTextureFlags {
    pub has_base_color: u32,
    pub has_metallic_roughness: u32,
    pub has_normal: u32,
    _pad: u32, // Padding for 16-byte alignment
}

impl From<&TextureFlags> for GpuTextureFlags {
    fn from(value: &TextureFlags) -> Self {
        Self {
            has_base_color: value.has_base_color,
            has_metallic_roughness: value.has_metallic_roughness,
            has_normal: value.has_normal,
            _pad: 0,
        }
    }
}

use wgpu::util::DeviceExt;

use crate::renderer::model::material::TextureFlags;

#[derive(Clone)]
pub struct PbrMaterial {
    pub bind_group: wgpu::BindGroup,
    material_buffer: wgpu::Buffer,
    tex_flags_buffer: wgpu::Buffer,
}

impl PbrMaterial {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        material_data: &Material,
        base_tex: (&wgpu::TextureView, &wgpu::Sampler),
        mr_tex: (&wgpu::TextureView, &wgpu::Sampler),
        normal_tex: (&wgpu::TextureView, &wgpu::Sampler),
    ) -> Self {
        let material_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::bytes_of(&GpuMaterial::from(material_data)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let tex_flags_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TextureFlags Buffer"),
            contents: bytemuck::bytes_of(&GpuTextureFlags::from(
                &material_data.get_texture_flags(),
            )),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: material_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: tex_flags_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(base_tex.0),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(base_tex.1),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(mr_tex.0),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(mr_tex.1),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(normal_tex.0),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::Sampler(normal_tex.1),
                },
            ],
            label: Some("PBR Material BindGroup"),
        });

        Self {
            bind_group,
            material_buffer,
            tex_flags_buffer,
        }
    }
}
