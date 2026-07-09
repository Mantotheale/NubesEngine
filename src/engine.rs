use crate::{
    fixed_timer::FixedTimer,
    math::point2f::Point2f,
    math::rect::Rect,
    renderer::color::Color,
    renderer::gl_context::vertex::layout::VertexLayout,
    renderer::gl_context::vertex::layout::VertexLayoutBuilder,
    renderer::gl_context::vertex::Vertex,
    renderer::Renderer
};
use std::time::{Duration, Instant};
use winit::window::WindowAttributes;
use winit::{
    event_loop::ActiveEventLoop,
    window::Window
};

pub struct Engine {
    renderer: Renderer,
    _window: Window,
    update_timer: FixedTimer,
    one_sec_timer: FixedTimer,
    update_count: u16,
    render_count: u16,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop, window_attributes: WindowAttributes) -> Self {
        let (renderer, window) = Renderer::new(event_loop, window_attributes);

        renderer.set_vsync(false);
        renderer.set_clear_color(Color::new(0.1, 0.2, 0.3, 1.0).unwrap());

        Self {
            renderer,
            _window: window,
            update_timer: FixedTimer::new(Duration::from_secs_f64(1f64 / 60f64)),
            one_sec_timer: FixedTimer::new(Duration::from_secs(1)),
            update_count: 0,
            render_count: 0
        }
    }

    pub fn update(&mut self) {
        self.update_count += 1;
    }

    pub fn render(&mut self) {
        self.renderer.clear();

        self.renderer.begin_scene();
        self.renderer.add_colored_rect(
            Rect::new(Point2f::new(0f32, 0f32), 1f32, 1f32).unwrap(),
            Color::new(0.75, 0.2, 0.3, 1f32).unwrap()
        );
        self.renderer.end_scene();

        self.renderer.swap_buffers();
        self.render_count += 1;
    }

    pub fn one_sec_update(&mut self) {
        println!("UPS: {}, FPS: {}", self.update_count, self.render_count);
        self.update_count = 0;
        self.render_count = 0;
    }

    pub fn tick(&mut self) {
        let current_time = Instant::now();

        while self.update_timer.try_tick(current_time) {
            self.update();
        }

        self.render();

        while self.one_sec_timer.try_tick(current_time) {
            self.one_sec_update();
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        println!("Engine dropped");
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::NoUninit)]
struct PositionVertex {
    x: f32,
    y: f32,
}

impl Vertex for PositionVertex {
    fn layout() -> VertexLayout {
        VertexLayoutBuilder::new().add_floats(2).build()
    }
}