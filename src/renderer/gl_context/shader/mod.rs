use std::rc::Rc;
use glow::NativeProgram;
use crate::{
    renderer::gl_context::GlContext,
    renderer::gl_context::shader::shader_kind::{FragmentShader, GeometryShader, VertexShader}
};

pub mod shader_kind;

pub struct ShaderProgram {
    native_program: NativeProgram,
    gl_context: Rc<GlContext>
}

impl ShaderProgram {
    pub fn new(
        gl_context: &Rc<GlContext>,
        vertex_shader: VertexShader, 
        geometry_shader: Option<GeometryShader>,
        fragment_shader: FragmentShader
    ) -> Self {
        let native_program = gl_context.gen_program(vertex_shader, geometry_shader, fragment_shader);
        
        Self { native_program, gl_context: gl_context.clone() }
    }
    
    pub fn native(&self) -> NativeProgram {
        self.native_program
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.gl_context.delete_program(self.native_program);
        println!("Program {:?} dropped", self.native_program);
    }
}