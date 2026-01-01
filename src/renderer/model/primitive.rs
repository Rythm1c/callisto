use wgpu::util::DeviceExt;

use crate::renderer::{model::importer::GltfFile, traits::Renderable, vertex::Vertex};

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    min: [f32; 3],
    max: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Primitive {
    vertex_buffer: wgpu::Buffer,
    index_buffer: Option<wgpu::Buffer>,
    index_count: usize,
    vertex_count: usize,
    bounds: BoundingBox,
    material: Option<usize>,
}

impl Primitive {
    pub fn get_bounds(&self) -> BoundingBox {
        self.bounds
    }

    pub fn get_vert_count(&self) -> usize {
        self.vertex_count
    }

    pub fn get_index_count(&self) -> usize {
        self.index_count
    }

    pub fn get_material(&self) -> Option<usize> {
        self.material
    }

    pub fn from_gltf(device: &wgpu::Device, primitive: &gltf::Primitive, file: &GltfFile) -> Self {
        let data = primitive_data_from_gltf(primitive, file);

        Self::upload(device, &data)
    }

    fn upload(device: &wgpu::Device, data: &PrimitiveData) -> Self {
        let vertex_count = data.vertices.len();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&data.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let mut index_buffer = None;
        let mut index_count = 0;
        if let Some(indices) = &data.indices {
            index_buffer = Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                }),
            );
            index_count = indices.len();
        }

        let bounds = data.bounds;
        let material = data.material;
        Primitive {
            vertex_buffer,
            index_buffer,
            index_count,
            vertex_count,
            bounds,
            material,
        }
    }
}

impl Renderable for Primitive {
    fn render(&self, renderpass: &mut wgpu::RenderPass) {
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        if let Some(index_buffer) = &self.index_buffer {
            renderpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            let count = self.index_count as u32;
            renderpass.draw_indexed(0..count, 0, 0..1);
        } else {
            let count = self.vertex_count as u32;
            renderpass.draw(0..count, 0..1);
        }
    }
}

struct PrimitiveData {
    vertices: Vec<Vertex>,
    indices: Option<Vec<u32>>,
    material: Option<usize>,
    mode: u32,
    bounds: BoundingBox,
}

fn primitive_data_from_gltf(primitive: &gltf::Primitive, file: &GltfFile) -> PrimitiveData {
    let mut material: Option<usize> = None;

    if let Some(mat) = primitive.material().index() {
        material = Some(mat);
    }
    let mode = primitive.mode().as_gl_enum();

    let mut vertices = Vec::new();
    let mut indices: Option<Vec<u32>> = None;

    let reader = primitive.reader(|buffer| Some(&file.get_buffers()[buffer.index()]));

    let bounds = BoundingBox {
        min: primitive.bounding_box().min,
        max: primitive.bounding_box().max,
    };

    if let Some(positions) = reader.read_positions() {
        positions.for_each(|position| {
            let mut vertex = Vertex::default();
            vertex.position = position;
            vertices.push(vertex)
        });
    } else {
        panic!(
            "a Primitive in folder {} does not contain the position attribute!",
            file.get_folder()
        );
    }

    if let Some(normals) = reader.read_normals() {
        normals
            .enumerate()
            .for_each(|(i, normal)| vertices[i].normal = normal);
    }

    if let Some(tex_coords) = reader.read_tex_coords(0) {
        tex_coords
            .into_f32()
            .enumerate()
            .for_each(|(i, uv)| vertices[i].tex_coord = uv);
    }

    /*  if let Some(weights) = reader.read_weights(0) {
        weights
            .into_f32()
            .enumerate()
            .for_each(|(i, weight_bach)| vertices[i].weights = weight_bach);
    }

    if let Some(joints) = reader.read_joints(0) {
        joints
            .into_u16()
            .enumerate()
            .for_each(|(i, joint_batch)| vertices[i].joints = joint_batch);
    } */

    if let Some(indices_reader) = reader.read_indices() {
        indices = Some(indices_reader.into_u32().collect::<Vec<u32>>());
    }

    PrimitiveData {
        vertices,
        indices,
        material,
        mode,
        bounds,
    }
}
