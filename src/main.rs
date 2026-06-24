pub mod app;
pub mod buffer;
pub mod core;
pub mod entity;
pub mod geometry;
pub mod scene;
pub mod vertex;

fn main() -> anyhow::Result<()> {
    app::run()?;
    Ok(())
}
