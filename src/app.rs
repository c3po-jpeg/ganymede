use std::sync::Arc;

use crate::{core::Core, entity::Entity, geometry::Geometry, scene::Scene};
use winit::{application::ApplicationHandler, window::Window};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    core: Option<Core>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let win_width = 800;
        let win_height = 600;
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

        let mut core = pollster::block_on(Core::new(window.clone())).unwrap();
        core.resize(win_width, win_height);
        self.core = Some(core);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let core = match &mut self.core {
            Some(core) => core,
            None => return,
        };

        match event {
            winit::event::WindowEvent::CloseRequested => {
                println!("closing app...");
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                let mut scene = Scene::new();
                scene.add_entity(Entity::new(core.device(), Geometry::triangle(None)));
                match core.render(|render_pass| scene.render(render_pass)) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
                // rendering code would go here
            }
            winit::event::WindowEvent::KeyboardInput { .. } => {
                // input handling code would go here
            }

            winit::event::WindowEvent::MouseInput { .. } => {
                // mouse input handling code would go here
            }

            winit::event::WindowEvent::Resized(size) => {
                core.resize(size.width, size.height);
            }
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut App::default())?;
    Ok(())
}
