use std::time::{Duration, Instant};
use glow::{Context, HasContext, ARRAY_BUFFER, COLOR_BUFFER_BIT, FLOAT, STATIC_DRAW, TRIANGLES};
use glutin::context::{GlProfile, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::GlWindow;
use raw_window_handle::HasWindowHandle;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use crate::fixed_timer::FixedTimer;

pub struct Engine {
    gl: Context,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
    _window: Window,
    update_timer: FixedTimer,
    one_sec_timer: FixedTimer,
    update_count: u16,
    render_count: u16
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
            use glutin::context::{ContextAttributesBuilder, ContextApi, Version};

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

        gl_surface
            .set_swap_interval(&gl_context, SwapInterval::DontWait)
            .expect("Couldn't set the swap interval");

        let vertex_array = unsafe {
            gl.create_vertex_array().expect("Cannot create vertex array")
        };
        unsafe {gl.bind_vertex_array(Some(vertex_array)); }

        let vertex_buffer = unsafe {
            gl.create_buffer().expect("Cannot create a vertex buffer")
        };
        unsafe { gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer)); }

        unsafe { gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&[
                -0.5f32, -0.5, 0.5, -0.5, -0.5, 0.5,
                0.5, -0.5, 0.5, 0.5, -0.5, 0.5
            ]),
            STATIC_DRAW
        ); }

        unsafe { gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, (2 * size_of::<f32>()) as i32, 0) }
        unsafe { gl.enable_vertex_array_attrib(vertex_array, 0); }
        let program = {
            let program = unsafe { gl.create_program().expect("Cannot create program") };

            let vertex_shader_source =
                "#version 330
                    layout(location = 0) in vec2 aPos;

                    out vec2 vColor;

                    void main() {
                        vColor = aPos;
                        gl_Position = vec4(aPos, 0.0, 1.0);
                    }";
            let fragment_shader_source =
                "#version 330
                    in vec2 vColor;

                    out vec4 fColor;

                    void main() {
                        fColor = vec4(vColor, 0.5, 1.0);
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

                unsafe { gl.shader_source(shader, &format!("{shader_source}")); }
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

        Self {gl,
            gl_surface,
            gl_context,
            _window: window,
            update_timer: FixedTimer::new(Duration::from_secs_f64(1f64 / 60f64)),
            one_sec_timer: FixedTimer::new(Duration::from_secs(1)),
            update_count: 0,
            render_count: 0
        }
    }

    pub fn update(&mut self) {
        self.update_count += 1;
    }

    pub fn render(&mut self) {
        unsafe { self.gl.clear(COLOR_BUFFER_BIT); }
        unsafe { self.gl.draw_arrays(TRIANGLES, 0, 6); }
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