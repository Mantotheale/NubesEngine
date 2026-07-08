use std::num::NonZero;
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

pub struct GlutinContext {
    glutin_surface: Surface<WindowSurface>,
    glutin_context: PossiblyCurrentContext,
}

impl GlutinContext {
    pub fn new(glutin_surface: Surface<WindowSurface>, glutin_context: PossiblyCurrentContext) -> Self {
        GlutinContext { glutin_surface, glutin_context }
    }
    
    pub fn swap_buffers(&self) {
        self.glutin_surface.swap_buffers(&self.glutin_context).expect("Couldn't swap buffers");
    }

    pub fn set_vsync(&self, is_vsync: bool) {
        if is_vsync {
            self.glutin_surface.set_swap_interval(
                &self.glutin_context,
                SwapInterval::Wait(NonZero::new(1).unwrap())
            ).expect("Couldn't set vsync");
        } else {
            self.glutin_surface.set_swap_interval(
                &self.glutin_context,
                SwapInterval::DontWait
            ).expect("Couldn't unset vsync");
        }
    }
}