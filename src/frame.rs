use crate::Pass;

pub struct Frame {
    texture: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
}

impl Frame {
    pub(crate) fn new(
        device: &wgpu::Device,
        surface: &wgpu::Surface,
    ) -> Result<Self, wgpu::SurfaceError> {
        let texture = surface.get_current_texture()?;
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        Ok(Self {
            texture,
            view,
            encoder,
        })
    }

    pub fn pass(&mut self, r: f32, g: f32, b: f32, a: f32) -> Pass {
        Pass::new(&mut self.encoder, &self.view, r, g, b, a)
    }

    pub(crate) fn finish(self, queue: &wgpu::Queue) {
        queue.submit(std::iter::once(self.encoder.finish()));
        self.texture.present();
    }
}
