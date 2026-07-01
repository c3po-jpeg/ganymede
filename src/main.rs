mod app;
mod buffer;
mod camera;
mod drawable;
mod entity;
mod renderer;
mod scene;

use entity::Entity;
use scene::Scene;

use geometry::{Geometry, Shape};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    scene.add_entity(Entity::new(Geometry::new(Shape::Cube {
        size: 1.0,
        color: None,
    })));
    app::run(scene)?;
    Ok(())
}
