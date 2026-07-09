use std::num::NonZero;
use glow::Context;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContext, PossiblyCurrentContext, Version};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::prelude::{GlConfig, GlSurface};
use glutin::surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

pub struct GlutinContext {
    glutin_display: Display,
    glutin_surface: Surface<WindowSurface>,
    glutin_context: PossiblyCurrentContext,
}

impl GlutinContext {
    pub fn new(event_loop: &ActiveEventLoop, window_attributes: WindowAttributes) -> (Self, Window) {
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, glutin_config) = {
            let glutin_template = ConfigTemplateBuilder::new();

            display_builder
                .build(event_loop, glutin_template, |configs| {
                    configs
                        .min_by_key(GlConfig::num_samples)
                        .expect("Couldn't find samples in the multisample buffer")
                })
                .expect("Couldn't build the OpenGLConfig")
        };

        let glutin_display = glutin_config.display();

        let raw_window_handle = window
            .as_ref()
            .and_then(|window| window.window_handle().map(Into::into).ok());

        let not_current_gl_context = {
            let context_attributes = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::OpenGl(Some(Version { major: 3, minor: 3, })))
                .with_profile(GlProfile::Core)
                .build(raw_window_handle);

            unsafe {
                glutin_display
                    .create_context(&glutin_config, &context_attributes)
                    .expect("Couldn't create a non current OpenGL context")
            }
        };

        let window = window.expect("Couldn't create a window");

        let glutin_surface = {
            let attrs = window.build_surface_attributes(
                SurfaceAttributesBuilder::default()
            ).expect("Couldn't build surface attributes");

            unsafe {
                glutin_display
                    .create_window_surface(&glutin_config, &attrs)
                    .expect("Couldn't create a surface")
            }
        };

        let glutin_context = not_current_gl_context
            .make_current(&glutin_surface)
            .expect("Couldn't make OpenGL context current");

        (Self { glutin_display, glutin_surface, glutin_context }, window)
    }

    pub fn load_glow_context(&self) -> Context {
        unsafe {
            Context::from_loader_function_cstr(|s| self.glutin_display.get_proc_address(s))
        }
    }
    
    pub fn swap_buffers(&self) {
        self.glutin_surface.swap_buffers(&self.glutin_context).expect("Couldn't swap buffers");
    }

    pub fn set_vsync(&self, is_vsync: bool) {
        if is_vsync {
            self.glutin_surface.set_swap_interval(
                &self.glutin_context,
                SwapInterval::Wait(NonZero::new(1).unwrap())
            ).expect("Couldn't set vsync");
        } else {
            self.glutin_surface.set_swap_interval(
                &self.glutin_context,
                SwapInterval::DontWait
            ).expect("Couldn't unset vsync");
        }
    }
}