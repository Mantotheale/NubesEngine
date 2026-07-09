use super::Vertex;
use crate::renderer::gl_context::{GlContext, GlPrimitive};
use glow::{Context, HasContext, NativeBuffer, NativeVertexArray, DYNAMIC_DRAW, STATIC_DRAW, UNSIGNED_BYTE, UNSIGNED_INT, UNSIGNED_SHORT};
use std::marker::PhantomData;
use std::rc::Rc;

pub trait Index: bytemuck::NoUninit {
    const GL_TYPE: u32;
}

impl Index for u8 {
    const GL_TYPE: u32 = UNSIGNED_BYTE;
}

impl Index for u16 {
    const GL_TYPE: u32 = UNSIGNED_SHORT;
}

impl Index for u32 {
    const GL_TYPE: u32 = UNSIGNED_INT;
}

pub enum GlUsageHint {
    StaticDraw,
    DynamicDraw
}

impl GlUsageHint {
    pub fn gl_value(&self) -> u32 {
        match self {
            GlUsageHint::StaticDraw => STATIC_DRAW,
            GlUsageHint::DynamicDraw => DYNAMIC_DRAW
        }
    }
}

struct CommonVertexArrayData<V: Vertex> {
    array: NativeVertexArray,
    vertex_buffer: NativeBuffer,
    capacity: usize,
    len: usize,
    phantom_data: PhantomData<V>,
}

impl<V: Vertex> CommonVertexArrayData<V> {
    fn new(gl_context: &GlContext, capacity: usize, usage: GlUsageHint) -> Self {
        let (vertex_array, vertex_buffer) =
            gl_context.gen_vertex_array::<V>(capacity, usage);
        Self { array: vertex_array, vertex_buffer, capacity, len: 0, phantom_data: PhantomData }
    }
}

struct IndexBufferData {
    buffer: NativeBuffer,
    len: usize,
    capacity: usize,
    gl_type: u32
}

impl IndexBufferData {
    fn new<I: Index>(gl_context: &GlContext, indices: &[I]) -> Self {
        let buffer = gl_context.gen_index_buffer(indices);
        Self { buffer, capacity: indices.len(), gl_type: I::GL_TYPE, len: 0 }
    }

    fn set_len(&mut self, len: usize) {
        assert!(len <= self.capacity);
        self.len = len;
    }
}

pub struct VertexArray<V: Vertex> {
    gl_context: Rc<GlContext>,
    common_data: CommonVertexArrayData<V>,
    index_data: Option<IndexBufferData>
}

impl<V: Vertex> VertexArray<V> {
    pub fn new(gl_context: &Rc<GlContext>, capacity: usize, usage: GlUsageHint) -> Self {
        Self {
            gl_context: gl_context.clone(),
            common_data: CommonVertexArrayData::new(gl_context, capacity, usage),
            index_data: None
        }
    }

    pub fn new_with_data(gl_context: &Rc<GlContext>, data: &[V], usage: GlUsageHint) -> Self {
        let mut array = Self::new(gl_context, data.len(), usage);
        array.load(data, None);
        array
    }

    pub fn new_indexed<I: Index>(gl_context: &Rc<GlContext>, capacity: usize, indices: &[I], usage: GlUsageHint) -> Self {
        let common_data = CommonVertexArrayData::new(gl_context, capacity, usage);
        let index_data = IndexBufferData::new(gl_context, indices);
        Self { gl_context: gl_context.clone(), common_data, index_data: Some(index_data) }
    }

    pub fn new_indexed_with_data<I: Index>(gl_context: &Rc<GlContext>, data: &[V], indices: &[I], usage: GlUsageHint, indices_len: usize) -> Self {
        let mut array = Self::new_indexed(gl_context, data.len(), indices, usage);
        array.load(data, Some(indices_len));
        array
    }

    pub fn native(&self) -> NativeVertexArray {
        self.common_data.array
    }

    pub fn load(&mut self, data: &[V], indices_len: Option<usize>) {
        assert!(data.len() <= self.common_data.capacity);
        self.gl_context.load_vertex_buffer(self.common_data.vertex_buffer, data);
        self.common_data.len = data.len();

        if let Some(indices_len) = indices_len {
            assert!(self.index_data.is_some());
            let Some(index_data) = &mut self.index_data else { unreachable!() };
            index_data.set_len(indices_len);
        }
    }

    pub fn draw(&self, gl: &Context, gl_primitive: GlPrimitive) {
        match &self.index_data {
            None => unsafe {
                gl.draw_arrays(gl_primitive.gl_value(), 0, self.common_data.len as i32);
            }
            Some(index_data) => unsafe {
                gl.draw_elements(
                    gl_primitive.gl_value(), index_data.len as i32, index_data.gl_type, 0
                );
            }
        }
    }
}

impl<V: Vertex> Drop for VertexArray<V> {
    fn drop(&mut self) {
        self.gl_context.delete_vertex_array(self.common_data.array);
        self.gl_context.delete_buffer(self.common_data.vertex_buffer);

        println!("Vao {:?} dropped", self.common_data.array);
        println!("Vbo {:?} dropped", self.common_data.vertex_buffer);

        if let Some(index_data) = &self.index_data {
            self.gl_context.delete_buffer(index_data.buffer);
            println!("Ebo {:?} dropped", index_data.buffer);

        }
    }
}