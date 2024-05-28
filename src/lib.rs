use window_state::WindowState;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

mod frame;
mod graphics_state;
mod window_state;
mod pass;
mod context;
mod buffer;
mod pipeline;

pub mod vertex;

pub use frame::*;
pub use pass::*;
pub use context::*;
pub use buffer::*;
pub use pipeline::*;

pub trait State {
    fn new(context: &mut Context) -> Self;
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
        if let Some(state) = self.window.as_ref() {
            state.request_redraw();
        }
    }
}

pub fn run<T: State>() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App::<T>::new()).unwrap();
}
