mod engine;
mod fixed_timer;
mod renderer;
mod constants;
mod math;

use crate::engine::Engine;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ControlFlow,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId
};
use winit::dpi::LogicalSize;
use winit::window::{Window, WindowAttributes};

fn main() {
    let window_attributes = Window::default_attributes()
        .with_title("Nubes Engine")
        .with_inner_size(LogicalSize::new(1280.0, 720.0));
    
    let event_loop = EventLoop::builder()
        .build()
        .expect("Couldn't build the event loop");

    event_loop.set_control_flow(ControlFlow::Poll);

    _ = event_loop.run_app(&mut EntryPoint::new(window_attributes));
}

struct EntryPoint {
    window_attributes: WindowAttributes,
    engine: Option<Engine>,
}

impl EntryPoint {
    fn new(window_attributes: WindowAttributes) -> Self {
        Self { engine: None, window_attributes }
    }
}

impl ApplicationHandler for EntryPoint {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = Some(Engine::new(event_loop, self.window_attributes.clone()));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let Some(engine) = &mut self.engine else { return; };
                engine.render();
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(engine) = &mut self.engine else { return; };
        engine.tick();
    }
}