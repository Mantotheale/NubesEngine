use crate::renderer::gl_context::vertex::array::{Index, VertexArray};
use glow::{Context, HasContext, NativeBuffer, NativeVertexArray, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, STATIC_DRAW, TRIANGLES};
use std::cell::RefCell;
use vertex::{array::GlUsageHint, Vertex};

pub mod vertex;

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
}

impl GlState {
    fn new() -> Self {
        Self {
            bound_vertex_array: None
        }
    }

    fn bind_vertex_array(&mut self, gl: &Context, array: NativeVertexArray) {
        if !matches!(self.bound_vertex_array, Some(a) if a == array) {
            unsafe { gl.bind_vertex_array(Some(array)); }
            self.bound_vertex_array = Some(array);
        }
    }

    fn unbind_vertex_array(&mut self, gl: &Context) {
        if self.bound_vertex_array.is_some() {
            unsafe { gl.bind_vertex_array(None); }
            self.bound_vertex_array = None;
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

    fn delete_vertex_array(&self, vertex_array: NativeVertexArray) {
        let mut state = self.state.borrow_mut();
        state.unbind_vertex_array(&self.gl);

        unsafe { self.gl.delete_vertex_array(vertex_array); }
    }

    fn delete_buffer(&self, buffer: NativeBuffer) {
        unsafe { self.gl.delete_buffer(buffer); }
    }

    pub fn draw<V: Vertex>(&self, vertex_array: &VertexArray<V>, gl_primitive: GlPrimitive) {
        let mut state = self.state.borrow_mut();
        state.bind_vertex_array(&self.gl, vertex_array.native());

        vertex_array.draw(&self.gl, gl_primitive);
    }
}