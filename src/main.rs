pub mod app;
pub mod core;

fn main() -> anyhow::Result<()> {
    app::run()?;
    Ok(())
}
