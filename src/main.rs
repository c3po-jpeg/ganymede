mod app;
mod buffer;
mod camera;
mod entity;
mod geometry;
mod renderer;
mod scene;
mod vertex;

use entity::Entity;
use geometry::Geometry;
use scene::Scene;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    scene.add_entity(Entity::new(Geometry::triangle(None)));
    app::run(scene)?;
    Ok(())
}
