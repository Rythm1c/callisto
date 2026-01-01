use crate::renderer::model::importer::GltfFile;

use super::primitive::Primitive;

#[derive(Clone)]
pub struct Mesh {
    primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn from_gltf(device: &wgpu::Device, mesh: &gltf::Mesh, file: &GltfFile) -> Self {
        Self {
            primitives: mesh
                .primitives()
                .map(|primitive| Primitive::from_gltf(device, &primitive, file))
                .collect::<Vec<Primitive>>(),
        }
    }

    pub fn get_primitive(&self, index: usize) -> &Primitive {
        &self.primitives[index]
    }

    pub fn primitive_count(&self) -> usize {
        self.primitives.len()
    }

    pub fn get_primitives(&self) -> &Vec<Primitive> {
        &self.primitives
    }

    pub fn get_vert_count(&self) -> usize {
        self.primitives
            .iter()
            .map(|primitive| primitive.get_vert_count())
            .sum()
    }

    pub fn get_index_count(&self) -> usize {
        self.primitives
            .iter()
            .map(|primitive| primitive.get_index_count())
            .sum()
    }
}
