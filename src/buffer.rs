use wgpu::util::DeviceExt;

pub struct Buffer {
    handle: wgpu::Buffer,
}

impl Buffer {
    pub fn new(device: &wgpu::Device, usage: wgpu::BufferUsages, data: &[u8]) -> Self {
        Self {
            handle: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data,
                usage,
            }),
        }
    }

    pub fn handle(&self) -> &wgpu::Buffer {
        &self.handle
    }
}
