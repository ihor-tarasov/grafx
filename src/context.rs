use crate::{
    data, uniform, vertex, BindGroup, BindGroupBuilder, Buffer, BufferAddress, Pipeline,
    PipelineBuilder, Sampler, Shader, Texture2D,
};

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

    pub fn shader(&self, src: String) -> Shader {
        Shader::new(&self.device, src)
    }

    pub fn pipeline(&self, builder: PipelineBuilder) -> Pipeline {
        Pipeline::new(&self.device, self.format, builder)
    }

    pub fn vertex_buffer<T: vertex::Vertex>(&self, data: &[T]) -> Buffer {
        Buffer::new_vertex(&self.device, data)
    }

    pub fn index_buffer<T: data::Pod>(&self, data: &[T]) -> Buffer {
        Buffer::new_index(&self.device, data)
    }

    pub fn uniform_buffer<T: uniform::Uniform>(&self, data: &[T]) -> Buffer {
        Buffer::new_uniform(&self.device, data)
    }

    pub fn sampler(&self) -> Sampler {
        Sampler::new(&self.device)
    }

    pub fn load_texture_2d(&self, data: &[u8]) -> Texture2D {
        Texture2D::load(&self.device, &self.queue, data)
    }

    pub fn bind_group(
        &self,
        bind_group_builder: BindGroupBuilder,
        pipeline_builder: &mut PipelineBuilder,
    ) -> BindGroup {
        bind_group_builder.build(&self.device, pipeline_builder)
    }

    pub fn write_buffer<T: uniform::Uniform>(
        &self,
        buffer: &Buffer,
        offset: BufferAddress,
        data: &[T],
    ) {
        buffer.write(&self.queue, offset, data)
    }
}
