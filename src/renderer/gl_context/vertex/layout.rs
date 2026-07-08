use glow::{Context, HasContext, NativeVertexArray, FLOAT};

pub use builder::VertexLayoutBuilder;

#[derive(Copy, Clone)]
pub enum GlType {
    Float
}

impl GlType {
    pub fn gl_type(&self) -> u32 {
        match self {
            GlType::Float => FLOAT
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            GlType::Float => 4
        }
    }

    pub unsafe fn vertex_attrib_pointer(&self, gl: &Context, index: usize, count: usize, stride: usize, offset: usize, ) {
        match self {
            GlType::Float => {
                unsafe {
                    gl.vertex_attrib_pointer_f32(
                        index as u32,
                        count as i32,
                        self.gl_type(),
                        false,
                        stride as i32,
                        offset as i32
                    )
                }
            }
        }
    }
}

pub struct VertexLayoutComponent {
    index: usize,
    count: usize,
    gl_type: GlType,
    stride: usize,
    offset: usize
}

impl VertexLayoutComponent {
    fn new(index: usize, count: usize, gl_type: GlType, stride: usize, offset: usize) -> Self {
        Self { index, count, gl_type, stride, offset }
    }

    fn byte_size(&self) -> usize {
        self.count * self.gl_type.byte_size()
    }

    pub unsafe fn enable_on(&self, gl: &Context, array: NativeVertexArray) {
        unsafe { self.gl_type.vertex_attrib_pointer(gl, self.index, self.count, self.stride, self.offset); }
        unsafe { gl.enable_vertex_array_attrib(array, self.index as u32); }
    }
}

pub struct VertexLayout {
    components: Vec<VertexLayoutComponent>,
    size: usize
}

impl VertexLayout {
    fn new(components: Vec<VertexLayoutComponent>) -> Self {
        let size = components.iter().map(|c| c.byte_size()).sum();
        Self { components, size }
    }

    pub fn byte_size(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> std::slice::Iter<'_, VertexLayoutComponent> {
        self.components.iter()
    }
}

impl<'a> IntoIterator for &'a VertexLayout {
    type Item = &'a VertexLayoutComponent;
    type IntoIter = std::slice::Iter<'a, VertexLayoutComponent>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

mod builder {
    use super::{GlType, VertexLayout, VertexLayoutComponent};

    struct VertexLayoutBuilderComponent {
        gl_type: GlType,
        count: usize,
    }

    impl VertexLayoutBuilderComponent {
        fn byte_size(&self) -> usize {
            self.gl_type.byte_size() * self.count
        }
    }

    pub struct VertexLayoutBuilder {
        components: Vec<VertexLayoutBuilderComponent>
    }

    impl VertexLayoutBuilder {
        pub fn new() -> Self {
            Self { components: Vec::new() }
        }

        pub fn add_float(mut self) -> Self {
            self.components.push(
                VertexLayoutBuilderComponent { gl_type: GlType::Float, count: 1}
            );
            self
        }

        pub fn add_floats(mut self, count: usize) -> Self {
            self.components.push(
                VertexLayoutBuilderComponent { gl_type: GlType::Float, count }
            );
            self
        }

        pub fn build(self) -> VertexLayout {
            let stride: usize = self.components.iter()
                .map(|c| c.byte_size())
                .sum();

            let mut offset = 0;
            let layout_components: Vec<VertexLayoutComponent> = self.components.iter()
                .enumerate()
                .map(|(index, component)| {
                    let c = VertexLayoutComponent::new(
                        index, component.count, component.gl_type, stride, offset
                    );

                    offset += component.byte_size();
                    c
                })
                .collect();

            VertexLayout::new(layout_components)
        }
    }
}