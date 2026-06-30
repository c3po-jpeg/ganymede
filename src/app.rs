use std::{sync::Arc, time::Instant};

use crate::{renderer::Renderer, scene::Scene};
use winit::{
    application::ApplicationHandler, event::ElementState, keyboard::PhysicalKey, window::Window,
};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    scene: Option<Scene>,
    last_frame_time: Option<Instant>,
    is_mouse_dragging: bool,
    last_mouse_pos: (f64, f64),
}

impl App {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene: Some(scene),
            ..Default::default()
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let win_width = 800u32;
        let win_height = 600u32;
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("ganymede")
                        .with_inner_size(winit::dpi::PhysicalSize::new(win_width, win_height)),
                )
                .unwrap(),
        );
        self.window = Some(window.clone());

        let mut renderer = pollster::block_on(Renderer::new(window.clone())).unwrap();
        renderer.resize(win_width, win_height);
        self.renderer = Some(renderer);

        if let Some(scene) = &mut self.scene {
            scene.init(self.renderer.as_ref().unwrap().device());
        } else {
            println!("scene has not been set!");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => return,
        };

        let scene = match &mut self.scene {
            Some(scene) => scene,
            None => return,
        };

        match event {
            winit::event::WindowEvent::CloseRequested => {
                println!("closing app...");
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                let dt = self
                    .last_frame_time
                    .map(|t| now.duration_since(t).as_secs_f32())
                    .unwrap_or(1.0 / 60.0);
                self.last_frame_time = Some(now);
                scene.update(dt);

                renderer.update_camera_buffer(scene.get_camera().get_ubo());
                match renderer.run(|render_pass| scene.render(render_pass)) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    scene.handle_keyboard(code, event.state == winit::event::ElementState::Pressed);
                }
            }

            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    match state {
                        ElementState::Pressed => self.is_mouse_dragging = true,
                        ElementState::Released => self.is_mouse_dragging = false,
                    }
                }
            }

            winit::event::WindowEvent::CursorMoved { position, .. } => {
                if self.is_mouse_dragging {
                    let dx = (position.x - self.last_mouse_pos.0) as f32;
                    let dy = (position.y - self.last_mouse_pos.1) as f32;

                    scene.rotate_camera(dx, dy);
                }
                self.last_mouse_pos = (position.x, position.y);
            }

            winit::event::WindowEvent::Resized(size) => {
                renderer.resize(size.width, size.height);
            }
            _ => {}
        }
    }
}

pub fn run(scene: Scene) -> anyhow::Result<()> {
    let mut app = App::new(scene);
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
