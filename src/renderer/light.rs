use crate::utils::color::ColorRGB;
use math::vec3::Vec3;

pub struct Light {
    position: Vec3,
    color: ColorRGB,
}

impl Light {
    pub fn new(position: Vec3, color: ColorRGB) -> Self {
        Self { position, color }
    }
}
