use std::sync::Arc;

use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
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
        }
    }

    pub fn event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.graphics.resize(size);
                self.user_state.resize(self.graphics.context_mut(), size.width as f32, size.height as f32);
            }
            WindowEvent::RedrawRequested => match self.graphics.render(&self.user_state) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => self.graphics.resize_own(),
                Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                Err(e) => log::error!("{e}"),
            },
            _ => {}
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
