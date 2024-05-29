pub use bytemuck::Pod;
pub use bytemuck::Zeroable;

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

pub trait Data {
    fn format() -> Format;
}

impl Data for [f32; 3] {
    fn format() -> Format {
        Format::Float32x3
    }
}

impl Data for [f32; 2] {
    fn format() -> Format {
        Format::Float32x2
    }
}
