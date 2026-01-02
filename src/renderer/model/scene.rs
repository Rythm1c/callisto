#[derive(Clone)]
pub struct Scene {
    pub nodes: Vec<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn from_gltf(scene: &gltf::Scene) -> Self {
        Self {
            nodes: scene
                .nodes()
                .map(|node| node.index())
                .collect::<Vec<usize>>(),
        }
    }
    pub fn get_node(&self, index: usize) -> &usize {
        &self.nodes[index]
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn set_nodes(&mut self, nodes: Vec<usize>) {
        self.nodes = nodes;
    }
}
