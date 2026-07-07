mod engine;
mod fixed_timer;

use crate::engine::Engine;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ControlFlow,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId
};

fn main() {
    let event_loop = EventLoop::builder()
        .build()
        .expect("Couldn't build the event loop");

    event_loop.set_control_flow(ControlFlow::Poll);

    _ = event_loop.run_app(&mut EntryPoint::new());
}

struct EntryPoint {
    engine: Option<Engine>,
}

impl EntryPoint {
    fn new() -> Self {
        Self { engine: None }
    }
}

impl ApplicationHandler for EntryPoint {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = Some(Engine::new(event_loop));
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