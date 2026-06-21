use winit::{application::ApplicationHandler, window::Window};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("ganymede")
                        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600)),
                )
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                println!("closing app...");
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                // rendering code would go here
            }
            winit::event::WindowEvent::KeyboardInput { .. } => {
                // input handling code would go here
            }

            winit::event::WindowEvent::MouseInput { .. } => {
                // mouse input handling code would go here
            }
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("WAYLAND_DISPLAY", "");
    }
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut App::default())?;
    Ok(())
}
