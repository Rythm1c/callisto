use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
}

pub fn vertex(pos: [f32; 3], norm: [f32; 3], tc: [f32; 2]) -> Vertex {
    Vertex {
        position: pos,
        normal: norm,
        tex_coord: tc,
    }
}

impl Vertex {
    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: (size_of::<f32>() * 3) as wgpu::BufferAddress,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: (size_of::<f32>() * 6) as wgpu::BufferAddress,
                    shader_location: 2,
                },
                /*    wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: (size_of::<f32>() * 3 * 3 * 2) as wgpu::BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint16x4,
                    offset: (size_of::<u32>() * 3 * 3 * 2 * 4) as wgpu::BufferAddress,
                    shader_location: 4,
                }, */
            ],
        }
    }
}
