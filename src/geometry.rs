use crate::buffer::Buffer;

use super::vertex::Vertex;

#[derive(Clone)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
}

impl Geometry {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices: Some(indices),
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn index_count(&self) -> usize {
        if let Some(indices) = &self.indices {
            indices.len()
        } else {
            0
        }
    }
}

pub struct GpuGeometry {
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, geometry: &Geometry) -> Self {
        let vertex_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&geometry.vertices),
        );
        let index_buffer = if let Some(indices) = &geometry.indices {
            Some(Buffer::new(
                device,
                wgpu::BufferUsages::INDEX,
                bytemuck::cast_slice(indices),
            ))
        } else {
            None
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
