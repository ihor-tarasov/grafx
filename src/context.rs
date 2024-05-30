use std::sync::Arc;

use glam::*;
use winit::{
    dpi::PhysicalPosition,
    window::{CursorGrabMode, Window},
};

pub struct Context {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
    window: Arc<Window>,
}

impl Context {
    pub(crate) async fn new(
        adapter: &wgpu::Adapter,
        format: wgpu::TextureFormat,
        window: Arc<Window>,
    ) -> Self {
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        Self {
            device,
            queue,
            format,
            window,
        }
    }

    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub(crate) fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub(crate) fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    pub fn size(&self) -> Vec2 {
        vec2(
            self.window.inner_size().width as f32,
            self.window.inner_size().height as f32,
        )
    }

    pub fn set_cursor_position(&self, pos: Vec2) {
        let _ = self
            .window
            .set_cursor_position(PhysicalPosition::new(pos.x, pos.y));
    }

    pub fn lock_cursor(&self) {
        let _ = self.window.set_cursor_grab(CursorGrabMode::Locked);
    }
}
