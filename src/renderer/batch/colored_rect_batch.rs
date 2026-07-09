use std::fmt;
use std::path::Path;
use std::rc::Rc;
use crate::constants;
use crate::math::rect::Rect;
use crate::renderer::batch::Colored2DVertex;
use crate::renderer::color::Color;
use crate::renderer::gl_context::{GlContext, GlPrimitive};
use crate::renderer::gl_context::shader::shader_kind::{FragmentShader, VertexShader};
use crate::renderer::gl_context::shader::ShaderProgram;
use crate::renderer::gl_context::vertex::array::{GlUsageHint, VertexArray};

#[derive(Debug)]
pub struct BatchFull;

impl fmt::Display for BatchFull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The colored rect batch is full (max {} rects)", constants::MAX_COLORED_RECTANGLES)
    }
}

impl std::error::Error for BatchFull {}

pub struct ColoredRectBatchData {
    vertex_array: VertexArray<Colored2DVertex>,
    shader_program: ShaderProgram,
    vertices_batch: Vec<Colored2DVertex>,
    inserted_rects: usize
}

impl ColoredRectBatchData {
    pub fn new(gl_context: &Rc<GlContext>) -> Self {
        const VERTICES_COUNT: usize = constants::MAX_COLORED_RECTANGLES * constants::VERTICES_PER_RECTANGLE;
        const INDICES_COUNT: usize = constants::MAX_COLORED_RECTANGLES * constants::INDICES_PER_RECTANGLE;

        let mut indices = [0u16; INDICES_COUNT];
        for i in 0..constants::MAX_COLORED_RECTANGLES {
            let index = i * constants::INDICES_PER_RECTANGLE;
            let vertex = (i * constants::VERTICES_PER_RECTANGLE) as u16;

            indices[index] = vertex;
            indices[index + 1] = vertex + 1;
            indices[index + 2] = vertex + 3;
            indices[index + 3] = vertex + 1;
            indices[index + 4] = vertex + 2;
            indices[index + 5] = vertex + 3;
        }

        let vertex_array = VertexArray::new_indexed(
            gl_context, VERTICES_COUNT, &indices, GlUsageHint::DynamicDraw
        );

        let shader_program = ShaderProgram::new(
            gl_context,
            VertexShader::new_from_path(gl_context, &Path::new(constants::ASSETS_PATH).join("shaders/simple_color.vert")),
            None,
            FragmentShader::new_from_path(gl_context, &Path::new(constants::ASSETS_PATH).join("shaders/simple_color.frag"))
        );

        ColoredRectBatchData {
            vertex_array,
            shader_program,
            vertices_batch: Vec::with_capacity(VERTICES_COUNT),
            inserted_rects: 0
        }
    }

    pub fn add_rect(&mut self, rect: Rect, color: Color) -> Result<(), BatchFull> {
        if self.inserted_rects == constants::MAX_COLORED_RECTANGLES { return Err(BatchFull); }

        let bottom_left = Colored2DVertex::from_point_and_color(rect.bottom_left(), color);
        let bottom_right = Colored2DVertex::from_point_and_color(rect.bottom_right(), color);
        let top_right = Colored2DVertex::from_point_and_color(rect.top_right(), color);
        let top_left = Colored2DVertex::from_point_and_color(rect.top_left(), color);

        self.vertices_batch.push(bottom_left);
        self.vertices_batch.push(bottom_right);
        self.vertices_batch.push(top_right);
        self.vertices_batch.push(top_left);

        self.inserted_rects += 1;
        Ok(())
    }

    pub fn flush(&mut self, gl_context: &GlContext) {
        self.vertex_array.load(
            self.vertices_batch.as_slice(), 
            Some(self.inserted_rects * constants::INDICES_PER_RECTANGLE)
        );
        gl_context.draw(&self.vertex_array, &self.shader_program, GlPrimitive::Triangles);
        
        self.vertices_batch.clear();
        self.inserted_rects = 0;
    }
}