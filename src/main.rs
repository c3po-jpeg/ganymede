pub mod app;
pub mod core;
pub mod geometry;
pub mod vertex;

fn main() -> anyhow::Result<()> {
    app::run()?;
    Ok(())
}
