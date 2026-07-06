use std::num::NonZeroU32;
use glow::{Context, HasContext, COLOR_BUFFER_BIT, TRIANGLES};
use glutin::context::PossiblyCurrentContext;
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::prelude::{GlSurface, NotCurrentGlContext};
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::GlWindow;
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

struct App {
    gl: Context,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => unsafe {
                self.gl.clear(COLOR_BUFFER_BIT);
                self.gl.draw_arrays(TRIANGLES, 0, 3);
                self.gl_surface.swap_buffers(&self.gl_context).unwrap();
            },
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::builder().build().expect("Couldn't build the event loop");

    let display_builder = {
        use winit::{dpi::LogicalSize, window::Window};
        use glutin_winit::DisplayBuilder;

        let window_builder = Window::default_attributes()
            .with_title("Nubes Engine")
            .with_inner_size(LogicalSize::new(1024.0, 768.0));

        DisplayBuilder::new().with_window_attributes(Some(window_builder))
    };

    let (window, gl_config) = {
        use glutin::config::{ConfigTemplateBuilder, GlConfig};

        let glutin_template = ConfigTemplateBuilder::new();
        display_builder
            .build(&event_loop, glutin_template, |configs| {
                configs
                    .max_by_key(GlConfig::num_samples)
                    .expect("Couldn't find samples in the multisample buffer")
            })
            .expect("Couldn't build the OpenGLConfig")
    };

    let gl_display = gl_config.display();

    let raw_window_handle = window
        .as_ref()
        .and_then(|window| window.window_handle().map(Into::into).ok());

    let not_current_gl_context = {
        use glutin::context::{ContextAttributesBuilder, ContextApi, Version};

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version { major: 3, minor: 3, })))
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

    gl_surface
        .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        .expect("Couldn't set the swap interval");

    let vertex_array = unsafe {
        gl.create_vertex_array().expect("Cannot create vertex array")
    };
    unsafe {gl.bind_vertex_array(Some(vertex_array)); }

    let program = {
        let program = unsafe { gl.create_program().expect("Cannot create program") };

        let vertex_shader_source =
            "const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }";
        let fragment_shader_source =
            "precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }";

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in &shader_sources {
            let shader = unsafe {
                gl.create_shader(*shader_type).expect("Cannot create shader")
            };

            unsafe { gl.shader_source(shader, &format!("#version 330\n{shader_source}")); }
            unsafe { gl.compile_shader(shader); }

            unsafe { assert!(gl.get_shader_compile_status(shader), "{}", gl.get_shader_info_log(shader)); }
            unsafe { gl.attach_shader(program, shader); }
            shaders.push(shader);
        }

        unsafe { gl.link_program(program); }
        unsafe { assert!(gl.get_program_link_status(program), "{}", gl.get_program_info_log(program)); }

        for shader in shaders {
            unsafe { gl.detach_shader(program, shader); }
            unsafe { gl.delete_shader(shader); }
        }

        program
    };

    unsafe { gl.use_program(Some(program)); }
    unsafe { gl.clear_color(0.1, 0.2, 0.3, 1.0); }

    _ = event_loop.run_app(&mut App {
        gl,
        gl_surface,
        gl_context,
    });
}