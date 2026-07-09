use crate::math::rect::Rect;
use crate::renderer::batch::colored_rect_batch::ColoredRectBatchData;
use crate::renderer::color::Color;
use crate::renderer::glutin_context::GlutinContext;
use gl_context::GlContext;
use std::rc::Rc;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

pub mod gl_context;
pub mod color;
mod glutin_context;
mod batch;

pub struct Renderer {
    glutin_context: GlutinContext,
    gl_context: Rc<GlContext>,
    colored_rect_batch_data: ColoredRectBatchData,
    has_scene_begun: bool
}

impl Renderer {
    pub fn new(event_loop: &ActiveEventLoop, window_attributes: WindowAttributes) -> (Self, Window) {
        let (glutin_context, window) = GlutinContext::new(event_loop, window_attributes);
        let gl_context = glutin_context.load_glow_context();
        let gl_context = Rc::new(GlContext::new(gl_context));
        let colored_rect_batch_data = ColoredRectBatchData::new(&gl_context);
        (Self { glutin_context, gl_context, colored_rect_batch_data, has_scene_begun: false }, window)
    }

    pub fn begin_scene(&mut self) {
        self.has_scene_begun = true;
    }

    pub fn end_scene(&mut self) {
        self.colored_rect_batch_data.flush(&self.gl_context);
        self.has_scene_begun = false;
    }

    pub fn add_colored_rect(&mut self, rect: Rect, color: Color) {
        if !self.has_scene_begun { panic!("Scene hasn't begun yet") }

        if self.colored_rect_batch_data.add_rect(rect, color).is_err() {
            self.colored_rect_batch_data.flush(&self.gl_context);
            self.colored_rect_batch_data.add_rect(rect, color)
                .expect("Batch was just flushed, so it must have room for one more rect");
        }
    }

    pub fn set_clear_color(&self, color: Color) {
        self.gl_context.set_clear_color(color);
    }

    pub fn clear(&self) {
        self.gl_context.clear();
    }

    pub fn set_blending(&self, is_blending: bool) {
        if is_blending {
            self.gl_context.enable_blending();
        } else {
            self.gl_context.disable_blending();
        }
    }

    pub fn swap_buffers(&self) {
        self.glutin_context.swap_buffers();
    }

    pub fn set_vsync(&self, is_vsync: bool) {
        self.glutin_context.set_vsync(is_vsync);
    }
}