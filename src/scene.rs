use crate::entity::Entity;

pub struct Scene {
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<()> {
        for entity in &self.entities {
            entity.render(render_pass)?;
        }
        Ok(())
    }
}
