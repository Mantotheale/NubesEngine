use std::fs;
use std::path::Path;
use std::rc::Rc;
use glow::{NativeShader, FRAGMENT_SHADER, GEOMETRY_SHADER, VERTEX_SHADER};
use crate::renderer::gl_context::GlContext;

pub struct VertexKind;
pub struct FragmentKind;
pub struct GeometryKind;

pub trait ShaderKind {
    const GL_TYPE: u32;
}

impl ShaderKind for VertexKind { const GL_TYPE: u32 = VERTEX_SHADER; }
impl ShaderKind for FragmentKind { const GL_TYPE: u32 = FRAGMENT_SHADER; }
impl ShaderKind for GeometryKind { const GL_TYPE: u32 = GEOMETRY_SHADER; }

pub struct Shader<K: ShaderKind> {
    native_shader: NativeShader,
    gl_context: Rc<GlContext>,
    _marker: std::marker::PhantomData<K>,
}

impl<K: ShaderKind> Shader<K> {
    pub fn new(gl_context: &Rc<GlContext>, source: &str) -> Self {
        let native_shader = gl_context.gen_shader::<K>(source);
        Self { native_shader, gl_context: gl_context.clone(), _marker: std::marker::PhantomData }
    }

    pub fn new_from_path(gl_context: &Rc<GlContext>, path: &Path) -> Self {
        let source = fs::read_to_string(path).expect("Couldn't read shader file");

        let native_shader = gl_context.gen_shader::<K>(&source);
        Self { native_shader, gl_context: gl_context.clone(), _marker: std::marker::PhantomData }
    }

    pub fn native(&self) -> NativeShader {
        self.native_shader
    }
}

impl<K: ShaderKind> Drop for Shader<K> {
    fn drop(&mut self) {
        self.gl_context.delete_shader(self.native_shader);
        println!("Shader {:?} dropped", self.native_shader);
    }
}

pub type VertexShader = Shader<VertexKind>;
pub type FragmentShader = Shader<FragmentKind>;
pub type GeometryShader = Shader<GeometryKind>;