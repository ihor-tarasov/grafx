use std::ops::RangeBounds;

use wgpu::util::DeviceExt;

use crate::{data, uniform, vertex, BindGroupEntry, BufferAddress};

pub use wgpu::IndexFormat;

pub struct BufferSlice<'a>(pub(crate) wgpu::BufferSlice<'a>);

pub struct Buffer(wgpu::Buffer);

impl Buffer {
    pub(crate) fn new_vertex<T: vertex::Vertex>(device: &wgpu::Device, data: &[T]) -> Self {
        Self(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        )
    }

    pub(crate) fn new_index<T: data::Pod>(device: &wgpu::Device, data: &[T]) -> Self {
        Self(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::INDEX,
            }),
        )
    }

    pub(crate) fn new_uniform<T: uniform::Uniform>(device: &wgpu::Device, data: &[T]) -> Self {
        Self(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
        )
    }

    pub(crate) fn write<T: uniform::Uniform>(&self, queue: &wgpu::Queue, offset: BufferAddress, data: &[T]) {
        queue.write_buffer(&self.0, offset, bytemuck::cast_slice(data));
    }

    pub fn slice<T: RangeBounds<BufferAddress>>(&self, bounds: T) -> BufferSlice {
        BufferSlice(self.0.slice(bounds))
    }
}

impl BindGroupEntry for Buffer {
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
