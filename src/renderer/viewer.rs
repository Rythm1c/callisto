use math::mat4::Mat4;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::renderer::camera::CameraMotion;
use crate::renderer::gui::GuiManager;

use super::camera::{Camera, CameraController};
use super::pipeline::Pipeline;
use super::traits::Renderable;

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
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    model_buffer: wgpu::Buffer,
    model_bind_group: wgpu::BindGroup,

    // GUI
    gui: GuiManager,
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

        // Create camera buffer
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&camera.view_projection_matrix().data.as_flattened()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera bind group"),
            layout: &pipeline.camera_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("model uniform buffer"),
            contents: bytemuck::cast_slice(&Mat4::IDENTITY.data.as_flattened()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("model bind group"),
            layout: &pipeline.model_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_buffer.as_entire_binding(),
            }],
        });

        let gui = GuiManager::new(&device, surface_format, &window);

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
            camera_buffer,
            camera_bind_group,
            // last_mouse_pos: (0.0, 0.0),
            model_buffer,
            model_bind_group,
            gui,
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
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    // ==================== Input Handling ====================
    pub fn handle_mouse_move(&mut self, delta_x: f32, delta_y: f32) {
        self.camera_controller
            .rotate(&mut self.camera, delta_x, delta_y);
        self.update_camera_buffer();
    }

    pub fn handle_window_event(&mut self, event: &winit::event::WindowEvent) {
        self.gui.handle_event(&self.window, event);
    }

    pub fn handle_keyboard(&mut self, key: winit::keyboard::KeyCode, pressed: bool) {
        if !pressed {
            CameraController::set_camera_motion(&mut self.camera, CameraMotion::Still);
            return; // Only handle key press, not release
        }

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

    fn update_camera_buffer(&mut self) {
        let view_proj = self.camera.view_projection_matrix();
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&view_proj.data.as_flattened()),
        );
    }

    pub fn update(&mut self, delta: f32) {
        // update camera data if theres motion detected
        if self.camera.motion != CameraMotion::Still {
            self.camera_controller
                .update_motion(&mut self.camera, delta);
            self.update_camera_buffer();
        }
    }

    // ==================== Rendering ====================
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        let aspect_ratio = new_size.width as f32 / new_size.height as f32;
        self.camera.aspect_ratio = aspect_ratio;

        self.configure_surface();
        self.depth_texture = Self::create_depth_texture(&self.device, new_size);
        self.update_camera_buffer();
    }

    pub fn render<T>(&mut self, model: &T)
    where
        T: Renderable,
    {
        // Begin GUI frame
        self.gui.begin_frame(&self.window);

        // Build UI
        self.gui.ui();

        // End GUI frame
        let egui_output = self.gui.end_frame();

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
                            r: 0.2,
                            g: 0.5,
                            b: 0.3,
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
            renderpass.set_bind_group(0, &self.camera_bind_group, &[]);
            renderpass.set_bind_group(1, &self.model_bind_group, &[]);

            model.render(&mut renderpass);
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
    }
}
