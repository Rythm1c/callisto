use math::{quaternion::Quat, transform::Transform, vec3::Vec3};

#[derive(Clone)]
pub struct Node {
    pub name: Option<String>,
    pub children: Vec<usize>,
    pub skin: Option<usize>,
    pub mesh: Option<usize>,
    pub transform: Transform,
}

impl Node {
    pub fn new() -> Self {
        Self {
            name: None,
            children: Vec::new(),
            skin: None,
            mesh: None,
            transform: Transform::DEFAULT,
        }
    }

    pub fn from_gltf(node: &gltf::Node) -> Self {
        let mut name: Option<String> = None;
        if let Some(n) = node.name() {
            name = Some(String::from(n));
        }

        let mut skin: Option<usize> = None;
        if let Some(s) = node.skin() {
            skin = Some(s.index());
        }

        let mut mesh: Option<usize> = None;
        if let Some(m) = node.mesh() {
            mesh = Some(m.index());
        }

        let mut transform = Transform::DEFAULT;
        let t = node.transform().decomposed();
        transform.translation = Vec3::from(&t.0);
        transform.orientation = Quat::from(&t.1);
        transform.scaling = Vec3::from(&t.2);

        Self {
            name,
            children: node
                .children()
                .map(|child| child.index())
                .collect::<Vec<usize>>(),
            skin,
            mesh,
            transform,
        }
    }

    pub fn with_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_mesh(&mut self, index: usize) {
        self.mesh = Some(index);
    }

    pub fn with_skin(&mut self, index: usize) {
        self.skin = Some(index);
    }

    pub fn with_children(&mut self, children: Vec<usize>) {
        self.children = children;
    }
}
