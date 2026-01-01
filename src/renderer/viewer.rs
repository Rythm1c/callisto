use math::mat4::{rotation_y, translate, transpose};
use math::quaternion::Quat;
use math::transform::Transform;
use math::vec3::vec3;
use std::path::Path;
use std::sync::Arc;
use winit::window::Window;

use crate::renderer::camera::CameraMotion;
use crate::renderer::gui::GuiManager;
use crate::renderer::light::DirectionalLight;
use crate::renderer::manager::RenderManager;
use crate::renderer::model::importer::GltfFile;
use crate::renderer::settings::ViewerSettings;
use crate::renderer::uniform::frame::{FrameBindGroup, FrameData};
use crate::renderer::uniform::model::{ModelBindGroup, ModelData};

use super::camera::{Camera, CameraController};
use super::pipeline::Pipeline;
use super::traits::Renderable;

use super::timer::Timer;

/// Central abstraction for rendering glTF assets and other models
pub struct Viewer {
    // Window and surface
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,

    // Rendering
    pipeline: Pipeline,
    depth_texture: wgpu::TextureView,

    // Camera system
    camera: Camera,
    camera_controller: CameraController,

    //uniforms
    frame_bind_group: FrameBindGroup,
    model_bind_group: ModelBindGroup,

    render_manager: RenderManager,

    //lighting
    sun: DirectionalLight,

    // GUI
    gui: GuiManager,

    settings: ViewerSettings,

    timer: Timer,
}

impl Viewer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let pipeline = Pipeline::new(&device, surface_format.into());
        let depth_texture = Self::create_depth_texture(&device, size);

        // Initialize camera
        let camera = Camera::new(aspect_ratio);
        let camera_controller = CameraController::new();

        let sun = DirectionalLight {
            direction: vec3(-1.0, -1.0, -1.0),
            color: [5.0, 5.0, 5.0],
        };

        let frame_bind_group =
            FrameBindGroup::new(&device, &pipeline.frame_layout, "Frame bind Group");
        let model_bind_group =
            ModelBindGroup::new(&device, &pipeline.model_layout, "Model bind group");

        let file = &GltfFile::load_gltf(Path::new("models/astronaut")).unwrap();

        let render_manager = RenderManager::new(&device, &pipeline.material_layout, &queue, file);

        let gui = GuiManager::new(&device, surface_format, &window);

        let timer = Timer::new();

        let settings = ViewerSettings::new([0.2, 0.5, 0.3]);

        let viewer = Self {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            pipeline,
            depth_texture,

            camera,
            camera_controller,

            sun,

            frame_bind_group,
            model_bind_group,

            render_manager,

            gui,

            settings,

            timer,
        };

        viewer.configure_surface();
        viewer
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    // ==================== Accessors ====================

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    // ==================== Input Handling ====================
    pub fn handle_mouse_move(&mut self, delta_x: f32, delta_y: f32) {
        if !self.gui.wants_mouse() {
            self.camera_controller
                .rotate(&mut self.camera, delta_x, delta_y);
            //self.update_camera_buffer();
        }
    }

    pub fn handle_window_event(&mut self, event: &winit::event::WindowEvent) {
        self.gui.handle_event(&self.window, event);
    }

    pub fn handle_keyboard(&mut self, key: winit::keyboard::KeyCode, pressed: bool) {
        if !pressed {
            CameraController::set_camera_motion(&mut self.camera, CameraMotion::Still);
            return; // Only handle key press, not release
        }

        if !self.gui.wants_keyboard() {
            match key {
                winit::keyboard::KeyCode::KeyW => {
                    CameraController::set_camera_motion(&mut self.camera, CameraMotion::Forwards);
                }
                winit::keyboard::KeyCode::KeyS => {
                    CameraController::set_camera_motion(&mut self.camera, CameraMotion::BackWards);
                }
                winit::keyboard::KeyCode::KeyA => {
                    CameraController::set_camera_motion(&mut self.camera, CameraMotion::Left);
                }
                winit::keyboard::KeyCode::KeyD => {
                    CameraController::set_camera_motion(&mut self.camera, CameraMotion::Right);
                }
                winit::keyboard::KeyCode::Space => {
                    CameraController::set_camera_motion(&mut self.camera, CameraMotion::Up);
                }
                winit::keyboard::KeyCode::ControlLeft | winit::keyboard::KeyCode::ControlRight => {
                    CameraController::set_camera_motion(&mut self.camera, CameraMotion::Down);
                }

                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.timer.update();
        // update camera data if theres motion detected
        if self.camera.motion != CameraMotion::Still {
            self.camera_controller
                .update_motion(&mut self.camera, self.timer.delta);
        }

        let frame_data = FrameData {
            view_proj: self.camera.view_projection_matrix(),
            cam_pos: self.camera.pos,
            sun: self.sun,
        };
        self.frame_bind_group.write(&self.queue, frame_data);

        let mut world = Transform::DEFAULT;
        world.orientation = Quat::rotation_y(180.0);
        world.translation = vec3(0.0, -2.0, 5.0);

        let model_data = ModelData {
            world: transpose(&world.to_mat()),
        };
        self.model_bind_group.write(&self.queue, model_data);
    }

    fn get_ui(&mut self) -> egui::FullOutput {
        // Begin GUI frame
        self.gui.begin_frame(&self.window);

        // Build UI
        egui::Window::new("Callisto viewer")
            .resizable(true)
            .show(&self.gui.get_ctx(), |ui| {
                ui.label("A glTF viewer built with Rust, wgpu, and egui");
                ui.separator();

                ui.monospace(format!("fps: {}", self.timer.fps()).as_str());
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("background color: ");
                    ui.color_edit_button_rgb(&mut self.settings.backgorund_color)
                });
                ui.separator();

                ui.label("Camera Info:");
                ui.monospace("Look around with mouse drag + movement keys");
            });
        // End GUI frame
        self.gui.end_frame()
    }

    // ==================== Rendering ====================
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        let aspect_ratio = new_size.width as f32 / new_size.height as f32;
        self.camera.aspect_ratio = aspect_ratio;

        self.configure_surface();
        self.depth_texture = Self::create_depth_texture(&self.device, new_size);
        self.gui.on_window_resized(&self.window);
    }

    pub fn render<F>(&mut self, render_func: F)
    where
        F: Fn(&mut wgpu::RenderPass),
    {
        // End GUI frame
        let egui_output = self.get_ui();

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.settings.backgorund_color[0] as f64,
                            g: self.settings.backgorund_color[1] as f64,
                            b: self.settings.backgorund_color[2] as f64,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            renderpass.set_pipeline(&self.pipeline.handle);
            renderpass.set_bind_group(0, &self.frame_bind_group.bind_group, &[]);
            renderpass.set_bind_group(1, &self.model_bind_group.bind_group, &[]);

            self.render_manager.render(&mut renderpass);

            render_func(&mut renderpass);
        }

        // Render GUI
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.size.width, self.size.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };

        self.gui.render(
            &self.device,
            &self.queue,
            &mut encoder,
            screen_descriptor,
            &texture_view,
            egui_output,
        );

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
        self.window.request_redraw();
    }
}
