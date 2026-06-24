use crate::geometry::{Geometry, GpuGeometry};

pub struct Entity {
    geometry: Geometry,
    drawable: GpuGeometry,
}

impl Entity {
    pub fn new(device: &wgpu::Device, geometry: Geometry) -> Self {
        Self {
            geometry: geometry.clone(),
            drawable: GpuGeometry::upload(device, &geometry),
        }
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<()> {
        render_pass.set_vertex_buffer(0, self.drawable.vertex_buffer().handle().slice(..));
        if let Some(index_buffer) = &self.drawable.index_buffer() {
            render_pass
                .set_index_buffer(index_buffer.handle().slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.geometry.index_count() as u32, 0, 0..1);
        } else {
            render_pass.draw(0..self.geometry.vertex_count() as u32, 0..1);
        }
        Ok(())
    }
}
