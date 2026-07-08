use crate::renderer::gl_context::vertex::array::{Index, VertexArray};
use glow::{Context, HasContext, NativeBuffer, NativeProgram, NativeShader, NativeVertexArray, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, STATIC_DRAW, TRIANGLES};
use std::cell::RefCell;
use vertex::{array::GlUsageHint, Vertex};
use crate::renderer::gl_context::shader::shader_kind::{FragmentShader, GeometryShader, ShaderKind, VertexShader};
use crate::renderer::gl_context::shader::ShaderProgram;

pub mod vertex;
pub mod shader;

pub enum GlPrimitive {
    Triangles
}

impl GlPrimitive {
    fn gl_value(&self) -> u32 {
        match self {
            GlPrimitive::Triangles => TRIANGLES
        }
    }
}

struct GlState {
    bound_vertex_array: Option<NativeVertexArray>,
    bound_program: Option<NativeProgram>
}

impl GlState {
    fn new() -> Self {
        Self {
            bound_vertex_array: None,
            bound_program: None
        }
    }

    fn bind_vertex_array(&mut self, gl: &Context, array: NativeVertexArray) {
        if !matches!(self.bound_vertex_array, Some(a) if a == array) {
            unsafe { gl.bind_vertex_array(Some(array)); }
            self.bound_vertex_array = Some(array);
        }
    }

    fn unbind_vertex_array(&mut self, gl: &Context, array: NativeVertexArray) {
        if self.bound_vertex_array.is_some_and(|a| a == array) {
            unsafe { gl.bind_vertex_array(None); }
            self.bound_vertex_array = None;
        }
    }

    fn bind_program(&mut self, gl: &Context, program: NativeProgram) {
        if !matches!(self.bound_program, Some(p) if p == program) {
            unsafe { gl.use_program(Some(program)); }
            self.bound_program = Some(program);
        }
    }

    fn unbind_program(&mut self, gl: &Context, program: NativeProgram) {
        if self.bound_program.is_some_and(|p| p == program) {
            unsafe { gl.use_program(None); }
            self.bound_program = None;
        }
    }
}

pub struct GlContext {
    gl: Context,
    state: RefCell<GlState>
}

impl GlContext {
    pub fn new(gl: Context) -> Self {
        Self { gl, state: RefCell::new(GlState::new()) }
    }

    pub fn as_raw(&self) -> &Context {
        &self.gl
    }

    fn gen_vertex_array<V: Vertex>(&self, len: usize, usage: GlUsageHint)
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

        unsafe { self.gl.buffer_data_size(ARRAY_BUFFER, (V::layout().byte_size() * len) as i32, usage.gl_value()); }

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
                "{}", self.gl.get_shader_info_log(shader)
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
                "{}", self.gl.get_program_info_log(program)
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