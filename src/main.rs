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
use crate::math::angle::Angle;
use crate::math::mat3f::Mat3f;
use crate::math::vec3f::Vec3f;

fn main() {
    let v = Vec3f::new(0f32, 1f32, 2f32);
    println!("{:?}", v);
    println!("{:?}", v * 3f32);
    println!("{:?}", 2f32 * v);
    println!("{:?}", v.len());
    println!("{:?}", v.normalize().expect("Non zero vector"));
    println!("{:?}", v + Vec3f::new(2.0, 3.4, 1.9));
    println!("{:?}", v - Vec3f::new(-1.0, 2.0, -4.0));

    let m = Mat3f::new(
        1f32, 0f32, 0f32,
        0f32, 1f32, 0f32,
        0f32, 0f32, 1f32
    );
    println!("{:?}", m);
    println!("{:?}", m[(1, 1)]);
    println!("{:?}", m.get(0, 2));
    println!("{:?}", m * 3.0);
    println!("{:?}", m - m);

    let m = Mat3f::new(
        0f32, 1f32, 0f32,
        0f32, 1f32, 1f32,
        2f32, 3f32, 4f32
    );
    let v = Vec3f::new(3f32, 5f32, 3f32);
    println!("{:?}", m * v);

    let v1 = Vec3f::new(1.0, 2.0, 3.0);
    let v2 = Vec3f::new(3.0, 2.0, 1.0);
    println!("{:?}", v1.cross(v2));
    let projection = v1.project(v2).expect("Non zero vectors");
    let rejection = v1.reject(v2).expect("Non zero vectors");
    println!("{:?}", projection);
    println!("{:?}", rejection);
    println!("{:?}", projection.dot(rejection));
    println!("{:?}", m.determinant());
    println!("{:?}", m.inverse().expect("Invertible matrix"));
    println!("{:?}", m * m.inverse().expect("Invertible matrix"));
    println!("{:?}", Angle::from_degrees(90.0).sin());
    println!("{:?}", Angle::from_degrees(-45.0).sin());
    println!("{:?}", Angle::from_degrees(90.0).cos());
    println!("{:?}", Angle::from_degrees(-45.0).cos());
    let v = Vec3f::new(1.0, 0.0, 0.0);
    let m = Mat3f::from_axis_rotation(
        Vec3f::new(0.0, 0.0, 1.0),
        Angle::from_degrees(90.0)
    ).expect("Non zero vector");
    println!("{:?}", m * v);

    /*let window_attributes = Window::default_attributes()
        .with_title("Nubes Engine")
        .with_inner_size(LogicalSize::new(1280.0, 720.0));
    
    let event_loop = EventLoop::builder()
        .build()
        .expect("Couldn't build the event loop");

    event_loop.set_control_flow(ControlFlow::Poll);

    _ = event_loop.run_app(&mut EntryPoint::new(window_attributes));*/
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