use image::RgbaImage;

use crate::{BindGroupEntry, Context};

pub struct Texture2D(wgpu::TextureView);

impl Texture2D {
    fn new_internal(device: &wgpu::Device, queue: &wgpu::Queue, image: RgbaImage) -> Self {
        let dimentions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimentions.0,
            height: dimentions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimentions.0),
                rows_per_image: Some(dimentions.1),
            },
            size,
        );
        Self(texture.create_view(&wgpu::TextureViewDescriptor::default()))
    }

    pub fn new(ctx: &Context, data: &[u8]) -> Self {
        let image = image::load_from_memory(data).unwrap().to_rgba8();
        Self::new_internal(ctx.device(), ctx.queue(), image)
    }
}

impl BindGroupEntry for Texture2D {
    fn visibility(&self) -> wgpu::ShaderStages {
        wgpu::ShaderStages::FRAGMENT
    }

    fn binding_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        }
    }

    fn resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::TextureView(&self.0)
    }
}

pub struct Sampler(wgpu::Sampler);

impl Sampler {
    pub fn new(ctx: &Context) -> Self {
        Self(ctx.device().create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        }))
    }
}

impl BindGroupEntry for Sampler {
    fn visibility(&self) -> wgpu::ShaderStages {
        wgpu::ShaderStages::FRAGMENT
    }

    fn binding_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
    }

    fn resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::Sampler(&self.0)
    }
}
