use crate::buffer::Buffer;

use geometry::Geometry;

pub struct Drawable {
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
}

impl Drawable {
    pub fn upload(device: &wgpu::Device, geometry: &Geometry) -> Self {
        let vertex_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&geometry.vertices()),
        );
        let index_buffer = if geometry.indices().is_empty() {
            None
        } else {
            Some(Buffer::new(
                device,
                wgpu::BufferUsages::INDEX,
                bytemuck::cast_slice(&geometry.indices()),
            ))
        };

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Option<Buffer> {
        &self.index_buffer
    }
}
