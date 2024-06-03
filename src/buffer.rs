use std::ops::RangeBounds;

use wgpu::util::DeviceExt;

use crate::{uniform, vertex, BindGroupEntry, BufferAddress, Context, Pass};

pub struct UniformBuffer(wgpu::Buffer);

impl UniformBuffer {
    pub fn new<T: uniform::Uniform>(ctx: &Context, data: &T) -> Self {
        Self(
            ctx.device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::bytes_of(data),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }),
        )
    }

    pub fn write<T: uniform::Uniform>(&self, ctx: &Context, offset: BufferAddress, data: &T) {
        ctx.queue()
            .write_buffer(&self.0, offset, bytemuck::bytes_of(data));
    }
}

impl BindGroupEntry for UniformBuffer {
    fn visibility(&self) -> wgpu::ShaderStages {
        wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT
    }

    fn binding_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    }

    fn resource(&self) -> wgpu::BindingResource {
        self.0.as_entire_binding()
    }
}

pub struct VertexBuffer(wgpu::Buffer);

impl VertexBuffer {
    pub fn new<T: vertex::Vertex>(ctx: &Context, data: &[T]) -> Self {
        Self(
            ctx.device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
        )
    }

    pub fn attach<'a, B: RangeBounds<BufferAddress>>(
        &'a self,
        pass: &mut Pass<'a>,
        slot: u32,
        bounds: B,
    ) {
        pass.0.set_vertex_buffer(slot, self.0.slice(bounds));
    }
}

pub struct IndexBufferU16(wgpu::Buffer);

impl IndexBufferU16 {
    pub fn new(ctx: &Context, data: &[u16]) -> Self {
        Self(
            ctx.device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        )
    }

    pub fn attach<'a, B: RangeBounds<BufferAddress>>(&'a self, pass: &mut Pass<'a>, bounds: B) {
        pass.0
            .set_index_buffer(self.0.slice(bounds), wgpu::IndexFormat::Uint16);
    }
}

pub struct IndexBufferU32(wgpu::Buffer);

impl IndexBufferU32 {
    pub fn new(ctx: &Context, data: &[u32]) -> Self {
        Self(
            ctx.device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        )
    }

    pub fn attach<'a, B: RangeBounds<BufferAddress>>(&'a self, pass: &mut Pass<'a>, bounds: B) {
        pass.0
            .set_index_buffer(self.0.slice(bounds), wgpu::IndexFormat::Uint32);
    }
}
