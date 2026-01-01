use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::ElementState,
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::Window,
};

use crate::renderer::Viewer;

struct App {
    viewer: Option<Viewer>,
    //sphere: Option<Primitive>,
    is_mouse_dragging: bool,
    last_mouse_pos: (f64, f64),
}

impl App {
    fn new() -> Self {
        Self {
            viewer: None,
            //sphere: None,
            is_mouse_dragging: false,
            last_mouse_pos: (0.0, 0.0),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("callisto - glTF Viewer"))
                .unwrap(),
        );

        let viewer = pollster::block_on(Viewer::new(window.clone()));

        // Initialize the cube primitive
        //let sphere: Primitive = Primitive::sphere(viewer.device(), &viewer.pipeline().model_layout);

        self.viewer = Some(viewer);
        // self.sphere = Some(sphere);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // Always pass events to the GUI first
        if let Some(viewer) = self.viewer.as_mut() {
            viewer.handle_window_event(&event);
        }

        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
                println!("Closing glTF viewer...");
            }

            winit::event::WindowEvent::RedrawRequested => {
                if let Some(viewer) = self.viewer.as_mut() {
                    viewer.update();
                    viewer.render();
                }
            }

            winit::event::WindowEvent::Resized(size) => {
                if let Some(viewer) = self.viewer.as_mut() {
                    viewer.resize(size);
                }
            }

            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    match state {
                        ElementState::Pressed => self.is_mouse_dragging = true,
                        ElementState::Released => self.is_mouse_dragging = false,
                    }
                    //self.is_mouse_dragging = state == winit::event::ElementState::Pressed;
                }
            }

            winit::event::WindowEvent::CursorMoved { position, .. } => {
                if self.is_mouse_dragging {
                    let delta_x = (position.x - self.last_mouse_pos.0) as f32;
                    let delta_y = (position.y - self.last_mouse_pos.1) as f32;

                    if let Some(viewer) = self.viewer.as_mut() {
                        viewer.handle_mouse_move(delta_x, delta_y);
                    }
                }
                self.last_mouse_pos = (position.x, position.y);
            }

            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    if let Some(viewer) = self.viewer.as_mut() {
                        viewer.handle_keyboard(
                            code,
                            event.state == winit::event::ElementState::Pressed,
                        );
                    }
                }
            }

            _ => {}
        }
    }
}

pub fn run() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run_app(&mut App::new())
        .expect("error occurred while running event loop");
}
