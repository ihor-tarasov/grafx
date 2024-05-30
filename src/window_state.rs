use std::{sync::Arc, time::Instant};

use glam::vec2;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

use crate::{
    graphics_state::{self, GraphicsState},
    State,
};

pub struct WindowState<T> {
    window: Arc<Window>,
    graphics: GraphicsState,
    user_state: T,
    last_time: Instant,
}

impl<T: State> WindowState<T> {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );
        let mut graphics = pollster::block_on(graphics_state::GraphicsState::new(window.clone()));
        let user_state = T::new(graphics.context_mut());
        Self {
            window,
            graphics,
            user_state,
            last_time: Instant::now(),
        }
    }

    pub fn event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.graphics.resize(size);
                self.user_state.resize(
                    self.graphics.context_mut(),
                    size.width as f32,
                    size.height as f32,
                );
            }
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => {
                    self.user_state
                        .key(self.graphics.context_mut(), code, event.state.is_pressed())
                }
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => self.user_state.cursor(
                self.graphics.context_mut(),
                vec2(position.x as f32, position.y as f32),
            ),
            WindowEvent::RedrawRequested => match self.graphics.render(&self.user_state) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => self.graphics.resize_own(),
                Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                Err(e) => log::error!("{e}"),
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now - self.last_time;
        self.last_time = now;
        self.user_state.update(self.graphics.context_mut(), delta);
        self.window.request_redraw();
    }
}
