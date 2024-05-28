pub use bytemuck::Pod;
pub use bytemuck::Zeroable;

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

#[derive(Clone, Copy)]
pub enum Format {
    Float32x2,
    Float32x3,
}

impl Into<wgpu::VertexFormat> for Format {
    fn into(self) -> wgpu::VertexFormat {
        match self {
            Format::Float32x2 => wgpu::VertexFormat::Float32x2,
            Format::Float32x3 => wgpu::VertexFormat::Float32x3,
        }
    }
}

pub trait Attribute {
    fn format() -> Format;
}

impl Attribute for [f32; 3] {
    fn format() -> Format {
        Format::Float32x3
    }
}

impl Attribute for [f32; 2] {
    fn format() -> Format {
        Format::Float32x2
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

    pub fn attribute<T: Attribute>(&mut self) {
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

pub unsafe trait Vertex: Pod + Zeroable {
    fn layout() -> Layout;
}

#[macro_export]
macro_rules! impl_vertex {
    {$visibility:vis struct $name:ident { $($field:ident: $field_type:ty),* $(,)?} } => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug)]
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

        unsafe impl $crate::vertex::Pod for $name {}
        unsafe impl $crate::vertex::Zeroable for $name {}
    };
}
