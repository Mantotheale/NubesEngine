use glow::{Context, HasContext, NativeProgram, NativeVertexArray, BLEND, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use crate::renderer::color::Color;

pub struct GlState {
    bound_vertex_array: Option<NativeVertexArray>,
    bound_program: Option<NativeProgram>,
    clear_color: Color,
    is_blending_enabled: bool
}

impl GlState {
    pub fn new() -> Self {
        Self {
            bound_vertex_array: None,
            bound_program: None,
            clear_color: Color::new(0f32, 0f32, 0f32, 0f32).expect("zero is in [0.0, 1.0]"),
            is_blending_enabled: false
        }
    }

    pub fn bind_vertex_array(&mut self, gl: &Context, array: NativeVertexArray) {
        if !matches!(self.bound_vertex_array, Some(a) if a == array) {
            unsafe { gl.bind_vertex_array(Some(array)); }
            self.bound_vertex_array = Some(array);
        }
    }

    pub fn unbind_vertex_array(&mut self, gl: &Context, array: NativeVertexArray) {
        if self.bound_vertex_array.is_some_and(|a| a == array) {
            unsafe { gl.bind_vertex_array(None); }
            self.bound_vertex_array = None;
        }
    }

    pub fn bind_program(&mut self, gl: &Context, program: NativeProgram) {
        if !matches!(self.bound_program, Some(p) if p == program) {
            unsafe { gl.use_program(Some(program)); }
            self.bound_program = Some(program);
        }
    }

    pub fn unbind_program(&mut self, gl: &Context, program: NativeProgram) {
        if self.bound_program.is_some_and(|p| p == program) {
            unsafe { gl.use_program(None); }
            self.bound_program = None;
        }
    }

    pub fn set_clear_color(&mut self, gl: &Context, color: Color) {
        unsafe { gl.clear_color(color.r(), color.g(), color.b(), color.a()); }
        self.clear_color = color;
    }

    pub fn enable_blending(&mut self, gl: &Context) {
        if !self.is_blending_enabled {
            unsafe { gl.enable(BLEND); }
            unsafe { gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA); }
            self.is_blending_enabled = true;
        }
    }

    pub fn disable_blending(&mut self, gl: &Context) {
        if self.is_blending_enabled {
            unsafe { gl.disable(BLEND); }
            self.is_blending_enabled = false;
        }
    }
}