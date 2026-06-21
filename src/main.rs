pub mod app;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    app::run()?;
    Ok(())
}
