use crate::data;

pub struct Layout {
    stride: wgpu::BufferAddress,
    attributes: Box<[wgpu::VertexAttribute]>,
}

impl Layout {
    pub(crate) fn as_vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: self.stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attributes,
        }
    }
}

pub struct LayoutBuilder {
    stride: wgpu::BufferAddress,
    attributes: Vec<wgpu::VertexAttribute>,
    location: u32,
    offset: wgpu::BufferAddress,
}

impl LayoutBuilder {
    pub fn new(stride: u64) -> Self {
        Self {
            stride,
            attributes: Vec::new(),
            location: 0,
            offset: 0,
        }
    }

    pub fn attribute<T: data::Data>(&mut self) {
        let format: wgpu::VertexFormat = T::format().into();

        let current_location = self.location;
        self.location += 1;

        let current_offset = self.offset;
        self.offset += format.size();

        self.attributes.push(wgpu::VertexAttribute {
            format,
            offset: current_offset,
            shader_location: current_location,
        })
    }

    pub fn build(self) -> Layout {
        Layout {
            stride: self.stride,
            attributes: self.attributes.into_boxed_slice(),
        }
    }
}

pub unsafe trait Vertex: data::Pod + data::Zeroable {
    fn layout() -> Layout;
}

#[macro_export]
macro_rules! impl_vertex {
    {$visibility:vis struct $name:ident { $($field:ident: $field_type:ty),* $(,)?} } => {
        #[repr(C)]
        #[derive(Clone, Copy)]
        $visibility struct $name {
            $(
                $field: $field_type,
            )*
        }

        unsafe impl $crate::vertex::Vertex for $name {
            fn layout() -> $crate::vertex::Layout {
                let mut builder = $crate::vertex::LayoutBuilder::new(std::mem::size_of::<$name>() as u64);
                $(
                    builder.attribute::<$field_type>();
                )*
                builder.build()
            }
        }

        unsafe impl $crate::data::Pod for $name {}
        unsafe impl $crate::data::Zeroable for $name {}
    };
}
