use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use crate::{Context, Frame, State};

async fn request_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface<'_>) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .unwrap()
}

fn create_surface_configuration(
    adapter: &wgpu::Adapter,
    surface: &wgpu::Surface,
    size: PhysicalSize<u32>,
) -> wgpu::SurfaceConfiguration {
    let caps = surface.get_capabilities(adapter);
    let format = caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(caps.formats[0]);
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: caps.present_modes[0],
        desired_maximum_frame_latency: 2,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
    }
}

pub struct GraphicsState {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    context: Context,
}

impl GraphicsState {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();
        let adapter = request_adapter(&instance, &surface).await;
        let config = create_surface_configuration(&adapter, &surface, size);
        let context = Context::new(&adapter, config.format).await;
        Self {
            surface,
            config,
            context,
        }
    }

    pub(crate) fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(self.context.device(), &self.config);
        }
    }

    pub fn resize_own(&mut self) {
        self.resize(PhysicalSize::new(self.config.width, self.config.height));
    }

    pub fn render<T: State>(&self, user_state: &T) -> Result<(), wgpu::SurfaceError> {
        let mut frame = Frame::new(self.context.device(), &self.surface)?;
        user_state.render(&mut frame);
        frame.finish(self.context.queue());
        Ok(())
    }
}
