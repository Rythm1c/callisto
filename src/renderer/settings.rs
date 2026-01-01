#[derive(Debug, Clone, Copy)]
pub struct ViewerSettings {
    pub backgorund_color: [f32; 3],
}

impl ViewerSettings {
    pub fn new(bg_color: [f32; 3]) -> Self {
        Self {
            backgorund_color: bg_color,
        }
    }
}
