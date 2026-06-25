use crate::{entity::Entity, geometry::Geometry, scene::Scene};

pub mod app;
pub mod buffer;
pub mod core;
pub mod entity;
pub mod geometry;
pub mod scene;
pub mod vertex;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    scene.add_entity(Entity::new(Geometry::triangle(None)));
    app::run(scene)?;
    Ok(())
}
