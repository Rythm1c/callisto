use egui_wgpu::{RendererOptions, ScreenDescriptor};
use winit::window::{Theme, Window};

/// GUI manager for the renderer
pub struct GuiManager {
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
}

impl GuiManager {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        window: &Window,
    ) -> Self {
        let egui_ctx = egui::Context::default();
        egui_ctx.set_pixels_per_point(window.scale_factor() as f32);

        let viewport_id = egui_ctx.viewport_id();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            viewport_id,
            window,
            Some(window.scale_factor() as f32),
            Some(Theme::Dark),
            Some(device.limits().max_texture_dimension_2d as usize),
        );

        let egui_renderer =
            egui_wgpu::Renderer::new(device, surface_format, RendererOptions::default());

        Self {
            egui_ctx,
            egui_state,
            egui_renderer,
        }
    }

    /// Handle window events (mouse, keyboard, etc.)
    pub fn handle_event(&mut self, window: &Window, window_event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_window_event(window, window_event);
    }

    /// Begin the GUI frame
    pub fn begin_frame(&mut self, window: &Window) {
        let raw_input = self.egui_state.take_egui_input(window);
        self.egui_ctx.begin_pass(raw_input);
    }

    pub fn get_ctx(&self) -> &egui::Context {
        &self.egui_ctx
    }

    pub fn wants_mouse(&self) -> bool {
        self.egui_state.egui_ctx().wants_pointer_input()
    }

    pub fn wants_keyboard(&self) -> bool {
        self.egui_state.egui_ctx().wants_keyboard_input()
    }

    /// Define your UI here
    pub fn ui(&self) {
        egui::Window::new("Callisto viewer")
            .resizable(true)
            .auto_sized()
            .show(&self.egui_ctx, |ui| {
                ui.heading("Callisto Viewer");
                ui.label("A glTF viewer built with Rust, wgpu, and egui");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Camera Info:");
                    ui.monospace("Look around with mouse drag + movement keys");
                });
            });
    }

    /// End frame and prepare rendering
    pub fn end_frame(&mut self) -> egui::FullOutput {
        self.egui_ctx.end_pass()
    }

    /// Render the GUI
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        screen_descriptor: ScreenDescriptor,
        window_surface_view: &wgpu::TextureView,
        full_output: egui::FullOutput,
    ) {
        let paint_jobs = self
            .egui_ctx
            .tessellate(full_output.shapes, self.egui_ctx.pixels_per_point());

        // Update texture delta
        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer
                .update_texture(device, queue, *id, image_delta);
        }

        self.egui_renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &screen_descriptor);
        // Tessellate

        // Render
        {
            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: window_surface_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.egui_renderer.render(
                &mut rpass.forget_lifetime(),
                &paint_jobs,
                &screen_descriptor,
            );
        }

        // Free unused textures
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
    }
}
