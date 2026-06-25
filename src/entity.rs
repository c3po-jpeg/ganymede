use crate::geometry::{Geometry, GpuGeometry};

pub struct Entity {
    geometry: Geometry,
    drawable: Option<GpuGeometry>,
}

impl Entity {
    pub fn new(geometry: Geometry) -> Self {
        Self {
            geometry: geometry.clone(),
            drawable: None,
        }
    }

    pub fn init(&mut self, device: &wgpu::Device) {
        self.drawable = Some(GpuGeometry::upload(device, &self.geometry));
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<()> {
        let drawable = match &self.drawable {
            Some(drawable) => drawable,
            None => anyhow::bail!("entity not initialized yet!"),
        };

        render_pass.set_vertex_buffer(0, drawable.vertex_buffer().handle().slice(..));
        if let Some(index_buffer) = &drawable.index_buffer() {
            render_pass
                .set_index_buffer(index_buffer.handle().slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.geometry.index_count() as u32, 0, 0..1);
        } else {
            render_pass.draw(0..self.geometry.vertex_count() as u32, 0..1);
        }
        Ok(())
    }
}
