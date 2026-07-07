use bytemuck::NoUninit;
use glow::{Context, HasContext, NativeBuffer, NativeVertexArray, ARRAY_BUFFER, FLOAT, STATIC_DRAW};

pub trait VertexLayout { }

pub trait Vertex<T: VertexLayout> { }

pub struct VertexArray<L: VertexLayout, T: Vertex<L>> {
    array: NativeVertexArray,
    buffer: NativeBuffer,
    _marker: std::marker::PhantomData<(L, T)>,
}

impl<L, T> VertexArray<L, T> where L: VertexLayout, T: Vertex<L> + NoUninit {
    pub fn new_with_size(gl: Context, size: usize) -> Self {
        let vertex_buffer = unsafe {
            gl.create_buffer().expect("Cannot create a vertex buffer")
        };
        unsafe { gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer)); }

        let vertex_array = unsafe {
            gl.create_vertex_array().expect("Cannot create vertex array")
        };
        unsafe {gl.bind_vertex_array(Some(vertex_array)); }

        unsafe { gl.buffer_data_size(ARRAY_BUFFER, size as i32, STATIC_DRAW); }

        unsafe { gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, (2 * size_of::<f32>()) as i32, 0) }
        unsafe { gl.enable_vertex_array_attrib(vertex_array, 0); }

        Self {
            array: vertex_array,
            buffer: vertex_buffer,
            _marker: std::marker::PhantomData
        }
    }

    pub fn new(gl: Context, data: &[T]) -> Self {
        let vertex_buffer = unsafe {
            gl.create_buffer().expect("Cannot create a vertex buffer")
        };
        unsafe { gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer)); }

        let vertex_array = unsafe {
            gl.create_vertex_array().expect("Cannot create vertex array")
        };
        unsafe {gl.bind_vertex_array(Some(vertex_array)); }

        unsafe { gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(data), STATIC_DRAW); }

        unsafe { gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, (2 * size_of::<f32>()) as i32, 0) }
        unsafe { gl.enable_vertex_array_attrib(vertex_array, 0); }

        Self {
            array: vertex_array,
            buffer: vertex_buffer,
            _marker: std::marker::PhantomData
        }
    }
}

pub struct PositionLayout { }

impl VertexLayout for PositionLayout { }

pub struct PositionVertex { x: f32, y: f32 }

impl Vertex<PositionLayout> for PositionVertex { }