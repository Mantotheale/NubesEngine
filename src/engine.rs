use std::path::Path;
use std::rc::Rc;
use crate::{
    fixed_timer::FixedTimer,
    renderer::gl_context::vertex::array::{GlUsageHint, VertexArray},
    renderer::gl_context::vertex::layout::{VertexLayout, VertexLayoutBuilder},
    renderer::gl_context::vertex::Vertex,
    renderer::gl_context::{GlContext, GlPrimitive}
};
use glow::{
    HasContext,
    COLOR_BUFFER_BIT
};
use glutin::{
    context::{GlProfile, NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{Surface, SwapInterval, WindowSurface}
};
use glutin_winit::GlWindow;
use raw_window_handle::HasWindowHandle;
use std::time::{Duration, Instant};
use winit::{
    event_loop::ActiveEventLoop,
    window::Window
};
use crate::renderer::gl_context::shader::shader_kind::{FragmentShader, VertexShader};
use crate::renderer::gl_context::shader::ShaderProgram;

pub struct Engine {
    gl: Rc<GlContext>,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
    _window: Window,
    update_timer: FixedTimer,
    one_sec_timer: FixedTimer,
    update_count: u16,
    render_count: u16,
    vertex_array1: VertexArray<PositionVertex>,
    vertex_array2: VertexArray<PositionVertex>,
    shader_program: ShaderProgram
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let display_builder = {
            use winit::{dpi::LogicalSize, window::Window};
            use glutin_winit::DisplayBuilder;

            let window_builder = Window::default_attributes()
                .with_title("Nubes Engine")
                .with_inner_size(LogicalSize::new(1280.0, 720.0));

            DisplayBuilder::new().with_window_attributes(Some(window_builder))
        };

        let (window, gl_config) = {
            use glutin::config::{ConfigTemplateBuilder, GlConfig};

            let glutin_template = ConfigTemplateBuilder::new();
            display_builder
                .build(event_loop, glutin_template, |configs| {
                    configs
                        .min_by_key(GlConfig::num_samples)
                        .expect("Couldn't find samples in the multisample buffer")
                })
                .expect("Couldn't build the OpenGLConfig")
        };

        let gl_display = gl_config.display();

        let raw_window_handle = window
            .as_ref()
            .and_then(|window| window.window_handle().map(Into::into).ok());

        let not_current_gl_context = {
            use glutin::context::{ContextApi, ContextAttributesBuilder, Version};

            let context_attributes = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::OpenGl(Some(Version { major: 3, minor: 3, })))
                .with_profile(GlProfile::Core)
                .build(raw_window_handle);

            unsafe {
                gl_display
                    .create_context(&gl_config, &context_attributes)
                    .expect("Couldn't create a non current OpenGL context")
            }
        };

        let window = window.expect("Couldn't create a window");

        let gl_surface = {
            use glutin::surface::SurfaceAttributesBuilder;

            let attrs = window.build_surface_attributes(
                SurfaceAttributesBuilder::default()
            ).expect("Couldn't build surface attributes");

            unsafe {
                gl_display
                    .create_window_surface(&gl_config, &attrs)
                    .expect("Couldn't create a surface")
            }
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Couldn't make OpenGL context current");

        let gl = unsafe {
            use glow::Context;
            Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s))
        };

        let gl = Rc::new(GlContext::new(gl));

        gl_surface
            .set_swap_interval(&gl_context, SwapInterval::DontWait)
            .expect("Couldn't set the swap interval");

        let vertices1 = vec![
            PositionVertex { x: -0.75, y: 0.5 },
            PositionVertex { x: 0.75, y: 0.5 },
            PositionVertex { x: 0f32, y: 0.75 },
        ];

        let vertices2 = vec![
            PositionVertex { x: -0.75, y: -0.75 },
            PositionVertex { x: 0.75, y: -0.75 },
            PositionVertex { x: 0.75, y: 0.5 },
            PositionVertex { x: -0.75, y: 0.5 },
        ];

        let indices = vec![0u8, 1, 3, 1, 2, 3];

        let vertex_array1 = VertexArray::new_with_data(&gl, &vertices1, GlUsageHint::StaticDraw);
        let vertex_array2 = VertexArray::new_indexed_with_data(&gl, &vertices2, &indices, GlUsageHint::StaticDraw);

        let vertex_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/basic.vert");
        let fragment_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/basic.frag");

        let shader_program = ShaderProgram::new(
            &gl,
            VertexShader::new_from_path(&gl, Path::new(vertex_path)),
            None,
            FragmentShader::new_from_path(&gl, Path::new(fragment_path))
        );

        unsafe { gl.as_raw().clear_color(0.1, 0.2, 0.3, 1.0); }

        Self {
            gl,
            gl_surface,
            gl_context,
            _window: window,
            update_timer: FixedTimer::new(Duration::from_secs_f64(1f64 / 60f64)),
            one_sec_timer: FixedTimer::new(Duration::from_secs(1)),
            update_count: 0,
            render_count: 0,
            vertex_array1,
            vertex_array2,
            shader_program
        }
    }

    pub fn update(&mut self) {
        self.update_count += 1;
    }

    pub fn render(&mut self) {
        unsafe { self.gl.as_raw().clear(COLOR_BUFFER_BIT); }
        self.gl.draw(&self.vertex_array1, &self.shader_program,  GlPrimitive::Triangles);
        self.gl.draw(&self.vertex_array2, &self.shader_program, GlPrimitive::Triangles);

        self.gl_surface.swap_buffers(&self.gl_context).expect("Couldn't swap buffers");

        self.render_count += 1;
    }

    pub fn one_sec_update(&mut self) {
        println!("UPS: {}, FPS: {}", self.update_count, self.render_count);
        self.update_count = 0;
        self.render_count = 0;
    }

    pub fn tick(&mut self) {
        let current_time = Instant::now();

        while self.update_timer.try_tick(current_time) {
            self.update();
        }

        self.render();

        while self.one_sec_timer.try_tick(current_time) {
            self.one_sec_update();
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        println!("Engine dropped");
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::NoUninit)]
struct PositionVertex {
    x: f32,
    y: f32,
}

impl Vertex for PositionVertex {
    fn layout() -> VertexLayout {
        VertexLayoutBuilder::new().add_floats(2).build()
    }
}