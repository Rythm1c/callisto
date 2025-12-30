use math::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct DirectionalLight {
    pub direction: Vec3,

    pub color: [f32; 3],
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: [f32; 3]) -> Self {
        Self { direction, color }
    }
}
