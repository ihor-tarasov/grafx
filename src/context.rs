use std::{collections::HashSet, sync::Arc};

use glam::*;
use winit::{
    dpi::PhysicalPosition,
    keyboard::KeyCode,
    window::{CursorGrabMode, Window},
};

pub struct Context {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
    window: Arc<Window>,
    keys: HashSet<KeyCode>,
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
            keys: HashSet::new(),
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

    pub(crate) fn set_key(&mut self, code: KeyCode, pressed: bool) {
        if pressed {
            self.keys.insert(code);
        } else {
            self.keys.remove(&code);
        }
    }

    pub fn key(&self, code: KeyCode) -> bool {
        self.keys.contains(&code)
    }
}
