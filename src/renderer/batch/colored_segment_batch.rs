use std::fmt;
use std::path::Path;
use std::rc::Rc;
use crate::constants;
use crate::math::point2f::Point2f;
use crate::math::segment::Segment;
use crate::renderer::color::Color;
use crate::renderer::gl_context::{GlContext, GlPrimitive};
use crate::renderer::gl_context::shader::shader_kind::{FragmentShader, GeometryShader, VertexShader};
use crate::renderer::gl_context::shader::ShaderProgram;
use crate::renderer::gl_context::vertex::array::{GlUsageHint, VertexArray};
use crate::renderer::gl_context::vertex::layout::{VertexLayout, VertexLayoutBuilder};
use crate::renderer::gl_context::vertex::Vertex;

#[derive(Debug)]
pub struct BatchFull;

impl fmt::Display for BatchFull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The colored rect batch is full (max {} rects)", constants::MAX_COLORED_RECTANGLES)
    }
}

impl std::error::Error for BatchFull {}

pub struct ColoredSegmentBatchData {
    vertex_array: VertexArray<Colored2DLineVertex>,
    shader_program: ShaderProgram,
    vertices_batch: Vec<Colored2DLineVertex>,
    inserted_lines: usize
}

impl ColoredSegmentBatchData {
    pub fn new(gl_context: &Rc<GlContext>) -> Self {
        const VERTICES_COUNT: usize = constants::MAX_COLORED_LINES * constants::VERTICES_PER_LINE;

        let vertex_array = VertexArray::new(
            gl_context, VERTICES_COUNT, GlUsageHint::DynamicDraw
        );

        let shader_program = ShaderProgram::new(
            gl_context,
            VertexShader::new_from_path(gl_context, &Path::new(constants::ASSETS_PATH).join("shaders/simple_line.vert")),
            Some(GeometryShader::new_from_path(gl_context, &Path::new(constants::ASSETS_PATH).join("shaders/simple_line.geom"))),
            FragmentShader::new_from_path(gl_context, &Path::new(constants::ASSETS_PATH).join("shaders/simple_line.frag"))
        );

        ColoredSegmentBatchData {
            vertex_array,
            shader_program,
            vertices_batch: Vec::with_capacity(VERTICES_COUNT),
            inserted_lines: 0
        }
    }

    pub fn add_segment(&mut self, segment: Segment, color: Color, pixel_width: f32) -> Result<(), BatchFull> {
        if self.inserted_lines== constants::MAX_COLORED_LINES { return Err(BatchFull); }

        self.vertices_batch.push(Colored2DLineVertex::from_point_color_width(
            segment.origin(), color, pixel_width
        ));
        self.vertices_batch.push(Colored2DLineVertex::from_point_color_width(
            segment.destination(), color, pixel_width
        ));

        self.inserted_lines += 1;
        Ok(())
    }

    pub fn flush(&mut self, gl_context: &GlContext) {
        self.vertex_array.load(self.vertices_batch.as_slice(), None);
        gl_context.draw(&self.vertex_array, &self.shader_program, GlPrimitive::Lines);

        self.vertices_batch.clear();
        self.inserted_lines = 0;
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::NoUninit)]
struct Colored2DLineVertex {
    x: f32, y: f32,
    r: f32, g: f32, b: f32, a: f32,
    width: f32
}

impl Colored2DLineVertex {
    pub fn from_point_color_width(point: Point2f, color: Color, width: f32) -> Self {
        Self { x: point.x(), y: point.y(), r: color.r(), g: color.g(), b: color.b(), a: color.a(), width }
    }
}

impl Vertex for Colored2DLineVertex {
    fn layout() -> VertexLayout {
        VertexLayoutBuilder::new()
            .add_floats(2)
            .add_floats(4)
            .add_float()
            .build()
    }
}