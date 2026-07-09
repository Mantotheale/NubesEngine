use crate::math::point2f::Point2f;
use crate::renderer::color::Color;
use crate::renderer::gl_context::vertex::layout::{VertexLayout, VertexLayoutBuilder};
use crate::renderer::gl_context::vertex::Vertex;

pub mod colored_rect_batch;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::NoUninit)]
struct Colored2DVertex {
    x: f32, y: f32,
    r: f32, g: f32, b: f32, a: f32
}

impl Colored2DVertex {
    pub fn from_point_and_color(point: Point2f, color: Color) -> Self {
        Self { x: point.x(), y: point.y(), r: color.r(), g: color.g(), b: color.b(), a: color.a() }
    }
}

impl Vertex for Colored2DVertex {
    fn layout() -> VertexLayout {
        VertexLayoutBuilder::new()
            .add_floats(2)
            .add_floats(4)
            .build()
    }
}