use std::time::{Duration, Instant};
use glium::backend::glutin::SimpleWindowBuilder;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use glium::winit::application::ApplicationHandler;
use glium::winit::dpi::{PhysicalPosition, PhysicalSize};
use glium::winit::event::{WindowEvent};
use glium::winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use glium::winit::window::{Window, WindowAttributes, WindowId};
use crate::engine::WakeUpEvent;

pub trait Application {
    fn new(window: Window, display: Display<WindowSurface>) -> Self;
    fn process_input(&mut self, input: WindowEvent);
    fn update(&mut self);
    fn fixed_update(&mut self);
    fn render(&mut self);
    fn should_close(&self) -> bool;
}

pub struct EngineLogic<T: Application> {
    app_state: Option<(T, NextUpdateTime)>,
    event_loop_proxy: EventLoopProxy<WakeUpEvent>,
}

impl<T: Application> EngineLogic<T> {
    pub fn new(event_loop_proxy: EventLoopProxy<WakeUpEvent>) -> Self {
        Self {
            app_state: None,
            event_loop_proxy,
        }
    }
}

impl<T: Application> ApplicationHandler<WakeUpEvent> for EngineLogic<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Nubes Engine")
            .with_inner_size(PhysicalSize::new(1280, 1024))
            .with_visible(false)
            .with_resizable(false);

        let (window, display) = SimpleWindowBuilder::new()
            .set_window_builder(window_attributes)
            .build(event_loop);

        let monitor = window.current_monitor().expect("Couldn't get the current monitor");
        let window_size = window.outer_size();
        let monitor_size = monitor.size();
        let monitor_position = monitor.position();

        let window_position = PhysicalPosition::<i32>::new(
            monitor_position.x + ((monitor_size.width - window_size.width) / 2) as i32,
            monitor_position.y + ((monitor_size.height - window_size.height) / 2) as i32
        );

        window.set_outer_position(window_position);
        window.set_visible(true);

        let app = T::new(window, display);
        let next_update_time = NextUpdateTime::new(
            Duration::from_secs_f64(1f64 / 60f64),
            Duration::from_secs(1)
        );
        self.app_state = Some((app, next_update_time));
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some((app, _)) = &mut self.app_state else { return; };

        match &event {
            WindowEvent::RedrawRequested => {
                app.render();
            }
            _ => { }
        }
        app.process_input(event)
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some((app, next_update_time)) = &mut self.app_state else { return; };
        if app.should_close() {
            event_loop.exit();
            return;
        }

        let current_time = Instant::now();
        loop {
            if !next_update_time.has_frame_expired(current_time) { break; }
            app.update();
            next_update_time.frame_tick();
        }

        app.render();

        loop {
            if !next_update_time.has_fixed_expired(current_time) { break; }
            app.fixed_update();
            next_update_time.fixed_tick();
        }
        self.event_loop_proxy.send_event(WakeUpEvent {}).expect("Couldn't send a self wake up event");
    }
}

struct NextUpdateTime {
    next_frame: Instant,
    next_fixed: Instant,
    frame_time: Duration,
    fixed_time: Duration
}

impl NextUpdateTime {
    pub fn new(frame_time: Duration, fixed_time: Duration) -> Self {
        let now = Instant::now();

        Self {
            next_frame: now + frame_time,
            next_fixed: now + fixed_time,
            frame_time,
            fixed_time
        }
    }

    pub fn frame_tick(&mut self) {
        self.next_frame = self.next_frame + self.frame_time;
    }

    pub fn fixed_tick(&mut self) {
        self.next_fixed = self.next_fixed + self.fixed_time;
    }

    pub fn has_frame_expired(&self, now: Instant) -> bool {
        now >= self.next_frame
    }

    pub fn has_fixed_expired(&self, now: Instant) -> bool {
        now >= self.next_fixed
    }
}