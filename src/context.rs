use crate::{buffer, pipeline, vertex};

pub struct Context {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
}

impl Context {
    pub(crate) async fn new(adapter: &wgpu::Adapter, format: wgpu::TextureFormat) -> Self {
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        Self {
            device,
            queue,
            format,
        }
    }

    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub(crate) fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn shader(&self, src: String) -> pipeline::Shader {
        pipeline::Shader::new(&self.device, src)
    }

    pub fn pipeline(&self, builder: pipeline::PipelineBuilder) -> pipeline::Pipeline {
        pipeline::Pipeline::new(&self.device, self.format, builder)
    }

    pub fn vertex_buffer<T: vertex::Vertex>(&self, data: &[T]) -> buffer::Buffer {
        buffer::Buffer::new_vertex(&self.device, data)
    }

    pub fn index_buffer<T: vertex::Pod>(&self, data: &[T]) -> buffer::Buffer {
        buffer::Buffer::new_index(&self.device, data)
    }
}
