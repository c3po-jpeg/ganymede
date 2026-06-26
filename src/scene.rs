use crate::{buffer::Buffer, camera::Camera, entity::Entity};

pub struct Scene {
    entities: Vec<Entity>,
    camera: Camera,
    camera_buffer: Option<Buffer>,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::new(800.0 / 600.0);

        Self {
            entities: Vec::new(),
            camera,
            camera_buffer: None,
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

    pub fn update(&mut self, dt: f32, aspect_ratio: f32) {}

    pub fn render(&self, render_pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<()> {
        for entity in &self.entities {
            entity.render(render_pass)?;
        }
        Ok(())
    }
}
