use crate::{camera::Camera, entity::Entity};

pub struct Scene {
    entities: Vec<Entity>,
    camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::new(800.0 / 600.0);

        Self {
            entities: Vec::new(),
            camera,
        }
    }

    pub fn init(&mut self, device: &wgpu::Device) {
        self.entities
            .iter_mut()
            .for_each(|entity| entity.init(device));
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn handle_keyboard(&mut self, key: winit::keyboard::KeyCode, pressed: bool) {
        if !pressed {
            self.camera.set_motion_still();
            return; // Only handle key press, not release
        }

        match key {
            winit::keyboard::KeyCode::KeyW => {
                self.camera.set_motion_forwards();
            }
            winit::keyboard::KeyCode::KeyS => {
                self.camera.set_motion_backwards();
            }
            winit::keyboard::KeyCode::KeyA => {
                self.camera.set_motion_left();
            }
            winit::keyboard::KeyCode::KeyD => {
                self.camera.set_motion_right();
            }
            winit::keyboard::KeyCode::Space => {
                self.camera.set_motion_up();
            }
            winit::keyboard::KeyCode::ControlLeft | winit::keyboard::KeyCode::ControlRight => {
                self.camera.set_motion_down();
            }

            _ => {}
        }
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn rotate_camera(&mut self, yaw: f32, pitch: f32) {
        self.camera.process_mouse(yaw, pitch);
    }

    pub fn update(&mut self, dt: f32) {
        self.camera.process_keyboard(dt);
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<()> {
        for entity in &self.entities {
            entity.render(render_pass)?;
        }
        Ok(())
    }
}
