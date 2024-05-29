use std::ops::RangeBounds;

use wgpu::util::DeviceExt;

use crate::{vertex, BufferAddress};

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

    pub(crate) fn new_index<T: vertex::Pod>(device: &wgpu::Device, data: &[T]) -> Self {
        Self(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::INDEX,
            }),
        )
    }

    pub fn slice<T: RangeBounds<BufferAddress>>(&self, bounds: T) -> BufferSlice {
        BufferSlice(self.0.slice(bounds))
    }
}
