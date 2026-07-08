use layout::VertexLayout;

pub mod layout;
pub mod array;

pub trait Vertex: bytemuck::NoUninit {
    fn layout() -> VertexLayout;
}

