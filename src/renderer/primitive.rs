use math::mat4::Mat4;
use wgpu::util::DeviceExt;

use crate::renderer::{
    traits::Renderable,
    vertex::{Vertex, vertex},
};

pub struct Primitive {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: usize,
    vertex_count: usize,
}

impl Primitive {
    pub fn cube(device: &wgpu::Device, model_layout: &wgpu::BindGroupLayout) -> Self {
        let (vertices, indices) = cube_vertices();
        Self::upload(device, model_layout, &vertices, &indices)
    }

    pub fn sphere(device: &wgpu::Device, model_layout: &wgpu::BindGroupLayout) -> Self {
        let (vertices, indices) = sphere_vertices(60, 60);
        Self::upload(device, model_layout, &vertices, &indices)
    }

    pub fn upload(
        device: &wgpu::Device,
        model_layout: &wgpu::BindGroupLayout,
        vertices: &Vec<Vertex>,
        indices: &Vec<u16>,
    ) -> Primitive {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Primitive {
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
            vertex_count: vertices.len(),
        }
    }
}

impl Renderable for Primitive {
    fn render(&self, renderpass: &mut wgpu::RenderPass) {
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        let count = self.index_count as u32;
        renderpass.draw_indexed(0..count, 0, 0..1);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModelUniform {
    pub transform: Mat4,
}

pub fn sphere_vertices(vertical_slices: i32, horizontal_slices: i32) -> (Vec<Vertex>, Vec<u16>) {
    let k = 180.0 / (vertical_slices - 1) as f32;
    let l = 360.0 / (horizontal_slices - 1) as f32;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Top vertex
    vertices.push(vertex([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]));

    // Middle vertices
    for i in 1..(vertical_slices - 1) {
        let theta = (i as f32 * k).to_radians();
        let y = theta.cos();
        let r = theta.sin();

        for j in 0..horizontal_slices {
            let phi = (j as f32 * l).to_radians();
            let x = r * phi.cos();
            let z = r * phi.sin();

            let normal = [x, y, z];
            vertices.push(vertex([x, y, z], normal));
        }
    }

    // Bottom vertex
    vertices.push(vertex([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]));

    // Top cap triangles
    for j in 0..horizontal_slices {
        let next_j = (j + 1) % horizontal_slices;

        indices.push((1 + j) as u16);
        indices.push((1 + next_j) as u16);
        indices.push(0);
    }

    // Middle triangles
    for i in 1..(vertical_slices - 2) {
        let curr_row = 1 + (i - 1) * horizontal_slices;
        let next_row = 1 + i * horizontal_slices;

        for j in 0..horizontal_slices {
            let next_j = (j + 1) % horizontal_slices;

            // First triangle
            indices.push((curr_row + next_j) as u16);
            indices.push((next_row + j) as u16);
            indices.push((curr_row + j) as u16);

            // Second triangle
            indices.push((next_row + next_j) as u16);
            indices.push((next_row + j) as u16);
            indices.push((curr_row + next_j) as u16);
        }
    }

    // Bottom cap triangles
    let bottom_vertex = (vertices.len() - 1) as u16;
    let last_row = 1 + (vertical_slices - 2) * horizontal_slices;

    for j in 0..horizontal_slices {
        let next_j = (j + 1) % horizontal_slices;
        indices.push((last_row + next_j) as u16);
        indices.push(bottom_vertex);
        indices.push((last_row + j) as u16);
    }

    (vertices, indices)
}

pub fn cube_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        vertex([-1.0, -1.0, 1.0], [0.0, 0.0, 1.0] /* [0.0, 0.0] */),
        vertex([1.0, -1.0, 1.0], [0.0, 0.0, 1.0] /*  [1.0, 0.0] */),
        vertex([1.0, 1.0, 1.0], [0.0, 0.0, 1.0] /*  [1.0, 1.0] */),
        vertex([-1.0, 1.0, 1.0], [0.0, 0.0, 1.0] /*  [0.0, 1.0] */),
        // bottom (0, 0, -1)
        vertex([-1.0, 1.0, -1.0], [0.0, 0.0, -1.0] /* [1.0, 0.0] */),
        vertex([1.0, 1.0, -1.0], [0.0, 0.0, -1.0] /* [0.0, 0.0] */),
        vertex([1.0, -1.0, -1.0], [0.0, 0.0, -1.0] /* [0.0, 1.0] */),
        vertex([-1.0, -1.0, -1.0], [0.0, 0.0, -1.0] /* [1.0, 1.0] */),
        // right (1, 0, 0)
        vertex([1.0, -1.0, -1.0], [1.0, 0.0, 0.0] /*  [0.0, 0.0] */),
        vertex([1.0, 1.0, -1.0], [1.0, 0.0, 0.0] /* [1.0, 0.0] */),
        vertex([1.0, 1.0, 1.0], [1.0, 0.0, 0.0] /* [1.0, 1.0] */),
        vertex([1.0, -1.0, 1.0], [1.0, 0.0, 0.0] /* [0.0, 1.0] */),
        // left (-1, 0, 0)
        vertex([-1.0, -1.0, 1.0], [-1.0, 0.0, 0.0] /* [1.0, 0.0] */),
        vertex([-1.0, 1.0, 1.0], [-1.0, 0.0, 0.0] /* [0.0, 0.0] */),
        vertex([-1.0, 1.0, -1.0], [-1.0, 0.0, 0.0] /* [0.0, 1.0] */),
        vertex([-1.0, -1.0, -1.0], [-1.0, 0.0, 0.0] /*  [1.0, 1.0] */),
        // front (0, 1, 0)
        vertex([1.0, 1.0, -1.0], [0.0, 1.0, 0.0] /*  [1.0, 0.0] */),
        vertex([-1.0, 1.0, -1.0], [0.0, 1.0, 0.0] /*  [0.0, 0.0] */),
        vertex([-1.0, 1.0, 1.0], [0.0, 1.0, 0.0] /* [0.0, 1.0] */),
        vertex([1.0, 1.0, 1.0], [0.0, 1.0, 0.0] /* [1.0, 1.0] */),
        // back (0, -1, 0)
        vertex([1.0, -1.0, 1.0], [0.0, -1.0, 0.0] /*  [0.0, 0.0] */),
        vertex([-1.0, -1.0, 1.0], [0.0, -1.0, 0.0] /*  [1.0, 0.0] */),
        vertex([-1.0, -1.0, -1.0], [0.0, -1.0, 0.0] /*  [1.0, 1.0] */),
        vertex([1.0, -1.0, -1.0], [0.0, -1.0, 0.0] /* [0.0, 1.0] */),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}
