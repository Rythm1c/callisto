pub trait Renderable {
    fn render(&self, renderpass: &mut wgpu::RenderPass);
}

