use std::rc::Rc;
use glutin::prelude::GlSurface;
use gl_context::GlContext;
use crate::renderer::gl_context::shader::ShaderProgram;
use crate::renderer::gl_context::vertex::array::VertexArray;
use crate::renderer::gl_context::vertex::layout::{VertexLayout, VertexLayoutBuilder};
use crate::renderer::gl_context::vertex::Vertex;
use crate::renderer::glutin_context::GlutinContext;

pub mod gl_context;
pub mod color;
mod glutin_context;

struct ColoredSquareBatchData {
    vertex_array: VertexArray<Colored2DVertex>,
    shader_program: ShaderProgram
}

impl ColoredSquareBatchData {
    pub fn new(gl_context: &Rc<GlContext>) -> Self {

    }
}

pub struct Renderer {
    glutin_context: GlutinContext,
    gl_context: GlContext,
    colored_square_batch_data: ColoredSquareBatchData
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::NoUninit)]
struct Colored2DVertex {
    x: f32, y: f32,
    r: f32, g: f32, b: f32, a: f32
}

impl Vertex for Colored2DVertex {
    fn layout() -> VertexLayout {
        VertexLayoutBuilder::new()
            .add_floats(2)
            .add_floats(4)
            .build()
    }
}