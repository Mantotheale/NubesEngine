use glium::Display;
use glium::glutin::surface::WindowSurface;
use glium::winit::event::WindowEvent;
use glium::winit::window::Window;
use crate::engine::engine_logic::Application;

pub struct Game {
    window: Window,
    display: Display<WindowSurface>,
    is_finished: bool
}

impl Application for Game {
    fn new(window: Window, display: Display<WindowSurface>) -> Self {
        Self {
            window,
            display,
            is_finished: false
        }
    }

    fn process_input(&mut self, input: WindowEvent) {
        if let WindowEvent::CloseRequested = input {
            self.is_finished = true;
        }
    }

    fn update(&mut self) {

    }

    fn fixed_update(&mut self) {
    }

    fn render(&mut self) {
    }

    fn should_close(&self) -> bool {
        self.is_finished
    }
}