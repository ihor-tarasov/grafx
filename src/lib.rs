use std::time::Duration;

pub use glam;
use glam::*;
use window_state::WindowState;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

mod bind_group;
mod buffer;
mod context;
mod frame;
mod graphics_state;
mod pass;
mod pipeline;
mod texture;
mod window_state;

pub mod data;
pub mod uniform;
pub mod vertex;

pub use bind_group::*;
pub use buffer::*;
pub use context::*;
pub use frame::*;
pub use pass::*;
pub use pipeline::*;
pub use texture::*;

pub type BufferAddress = wgpu::BufferAddress;
pub type DynamicOffset = wgpu::DynamicOffset;

pub use winit::keyboard::KeyCode;

pub trait State {
    fn new(context: &Context) -> Self;
    fn resize(&mut self, _ctx: &Context, _size: Vec2) {}
    fn key(&mut self, _ctx: &Context, _code: KeyCode, _pressed: bool) {}
    fn cursor(&mut self, _ctx: &Context, _pos: Vec2) {}
    fn update(&mut self, _ctx: &Context, _delta: Duration) {}
    fn render(&self, frame: &mut Frame);
}

struct App<T> {
    window: Option<WindowState<T>>,
}

impl<T> App<T> {
    fn new() -> Self {
        Self { window: None }
    }
}

impl<T: State> ApplicationHandler for App<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(window_state::WindowState::new(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = self.window.as_mut() {
            window.event(event_loop, event);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = self.window.as_mut() {
            state.update();
        }
    }
}

pub fn run<T: State>() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App::<T>::new()).unwrap();
}
