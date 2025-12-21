use glium::winit::event_loop::{ControlFlow, EventLoop};
use crate::engine::engine_logic::{Application, EngineLogic};

mod engine_logic;
pub mod game;

#[derive(Debug)]
pub struct WakeUpEvent { }

pub struct Engine<T: Application> {
    event_loop: EventLoop<WakeUpEvent>,
    engine_logic: EngineLogic<T>
}

impl<T: Application> Engine<T> {
    pub fn new() -> Self {
        let event_loop = EventLoop::with_user_event().build().expect("Couldn't build the event loop");
        let event_loop_proxy = event_loop.create_proxy();

        Self {
            event_loop,
            engine_logic: EngineLogic::new(event_loop_proxy)
        }
    }

    pub fn run(mut self) {
        self.event_loop.set_control_flow(ControlFlow::Poll);
        self.event_loop.run_app(&mut self.engine_logic).expect("Event loop exited with an exception");
    }
}