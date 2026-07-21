use crate::renderer::color::Color;
use crate::renderer::gl_context::gl_state::GlState;
use crate::renderer::gl_context::shader::shader_kind::{FragmentShader, GeometryShader, ShaderKind, VertexShader};
use crate::renderer::gl_context::shader::ShaderProgram;
use crate::renderer::gl_context::vertex::array::{Index, VertexArray};
use glow::{Context, HasContext, NativeBuffer, NativeProgram, NativeShader, NativeVertexArray, ARRAY_BUFFER, COLOR_BUFFER_BIT, ELEMENT_ARRAY_BUFFER, LINES, STATIC_DRAW, TRIANGLES};
use std::cell::RefCell;
use vertex::{array::GlUsageHint, Vertex};

pub mod vertex;
pub mod shader;
mod gl_state;

pub struct GlContext {
    gl: Context,
    state: RefCell<GlState>
}

impl GlContext {
    pub fn new(gl: Context) -> Self {
        Self { gl, state: RefCell::new(GlState::new()) }
    }
    
    fn gen_vertex_array<V: Vertex>(&self, capacity: usize, usage: GlUsageHint)
                                   -> (NativeVertexArray, NativeBuffer)
    {
        let mut state = self.state.borrow_mut();

        let vertex_array = unsafe {
            self.gl.create_vertex_array().expect("Cannot create vertex array")
        };
        state.bind_vertex_array(&self.gl, vertex_array);

        let vertex_buffer = unsafe {
            self.gl.create_buffer().expect("Cannot create a vertex buffer")
        };
        unsafe { self.gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer)); }

        unsafe { self.gl.buffer_data_size(ARRAY_BUFFER, (V::layout().byte_size() * capacity) as i32, usage.gl_value()); }

        for component in &V::layout() {
            unsafe { component.enable_on(&self.gl, vertex_array); }
        }

        (vertex_array, vertex_buffer)
    }

    fn gen_index_buffer<I: Index>(&self, indices: &[I]) -> NativeBuffer {
        let index_buffer = unsafe {
            self.gl.create_buffer().expect("Couldn't create an index buffer")
        };

        unsafe { self.gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(index_buffer)); }
        unsafe { self.gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(indices), STATIC_DRAW); }

        index_buffer
    }

    fn load_vertex_buffer<V: Vertex>(&self, vertex_buffer: NativeBuffer, data: &[V]) {
        unsafe { self.gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer)); }
        unsafe { self.gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, 0, bytemuck::cast_slice(data)); }
    }

    fn gen_shader<K: ShaderKind>(&self, source: &str) -> NativeShader {
        let shader = unsafe {
            self.gl.create_shader(K::GL_TYPE).expect("Cannot create shader")
        };

        unsafe { self.gl.shader_source(shader, source); }
        unsafe { self.gl.compile_shader(shader); }

        unsafe {
            assert!(
                self.gl.get_shader_compile_status(shader),
                "{} shader failed to compile:\n{}", K::NAME, self.gl.get_shader_info_log(shader)
            );
        }

        shader
    }

    fn gen_program(
        &self,
        vertex_shader: VertexShader,
        geometry_shader: Option<GeometryShader>,
        fragment_shader: FragmentShader
    ) -> NativeProgram {
        let program = unsafe {
            self.gl.create_program().expect("Cannot create program")
        };

        unsafe { self.gl.attach_shader(program, vertex_shader.native()); }
        unsafe { self.gl.attach_shader(program, fragment_shader.native()); }
        if let Some(s) = geometry_shader.as_ref() {
            unsafe { self.gl.attach_shader(program, s.native()); }
        }

        unsafe { self.gl.link_program(program); }

        unsafe {
            assert!(
                self.gl.get_program_link_status(program),
                "shader program failed to link:\n{}", self.gl.get_program_info_log(program)
            );
        }

        unsafe { self.gl.detach_shader(program, vertex_shader.native()); }
        unsafe { self.gl.detach_shader(program, fragment_shader.native()); }
        if let Some(s) = geometry_shader {
            unsafe { self.gl.detach_shader(program, s.native()); }
        }

        program
    }

    pub fn draw<V: Vertex>(&self, vertex_array: &VertexArray<V>, program: &ShaderProgram, gl_primitive: GlPrimitive) {
        let mut state = self.state.borrow_mut();
        state.bind_vertex_array(&self.gl, vertex_array.native());
        state.bind_program(&self.gl, program.native());

        vertex_array.draw(&self.gl, gl_primitive);
    }

    pub fn set_clear_color(&self, color: Color) {
        self.state.borrow_mut().set_clear_color(&self.gl, color);
    }

    pub fn clear(&self) {
        unsafe { self.gl.clear(COLOR_BUFFER_BIT); }
    }

    pub fn enable_blending(&self) {
        self.state.borrow_mut().enable_blending(&self.gl);
    }

    pub fn disable_blending(&self) {
        self.state.borrow_mut().disable_blending(&self.gl);
    }

    fn delete_vertex_array(&self, vertex_array: NativeVertexArray) {
        let mut state = self.state.borrow_mut();
        state.unbind_vertex_array(&self.gl, vertex_array);

        unsafe { self.gl.delete_vertex_array(vertex_array); }
    }

    fn delete_buffer(&self, buffer: NativeBuffer) {
        unsafe { self.gl.delete_buffer(buffer); }
    }

    fn delete_shader(&self, shader: NativeShader) {
        unsafe { self.gl.delete_shader(shader); }
    }

    fn delete_program(&self, program: NativeProgram) {
        let mut state = self.state.borrow_mut();
        state.unbind_program(&self.gl, program);

        unsafe { self.gl.delete_program(program); }
    }
}

pub enum GlPrimitive {
    Triangles,
    Lines
}

impl GlPrimitive {
    fn gl_value(&self) -> u32 {
        match self {
            GlPrimitive::Triangles => TRIANGLES,
            GlPrimitive::Lines => LINES
        }
    }
}