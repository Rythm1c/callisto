#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn color_rgb(r: f32, g: f32, b: f32) -> ColorRGB {
    ColorRGB { r, g, b }
}
