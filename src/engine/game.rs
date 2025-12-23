use crate::engine::engine_logic::Application;
use glium::glutin::surface::WindowSurface;
use glium::texture::RawImage2d;
use glium::winit::event::WindowEvent;
use glium::winit::window::Window;
use glium::{implement_vertex, uniform, Display, IndexBuffer, Program, Surface, Texture2d, VertexBuffer};
use image::ImageReader;
use std::fmt::Formatter;
use std::path::Path;
use std::{env, fmt, io};
use glium::index::PrimitiveType::TrianglesList;

pub struct Game {
    _window: Window,
    display: Display<WindowSurface>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program,
    texture: Texture2d,
    is_finished: bool,
    update_count: u16,
    render_count: u16
}

impl Application for Game {
    fn new(window: Window, display: Display<WindowSurface>) -> Self {
        let vertices = [
            Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 1.0] },
            Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
            Vertex { position: [0.5, -0.5], tex_coords: [1.0, 0.0] },
            Vertex { position: [0.5, 0.5], tex_coords: [1.0, 1.0] }
        ];

        let indices = [
            0, 1, 3,
            1, 2, 3
        ];

        let vertex_buffer = VertexBuffer::new(&display, &vertices).expect("Couldn't create a vertex buffer");
        let index_buffer = IndexBuffer::new(&display, TrianglesList, &indices).expect("Couldn't create an index buffer");

        let vertex_shader_src = r#"
            #version 330

            in vec2 position;
            in vec2 tex_coords;

            out vec2 v_tex_coords;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;


        let fragment_shader_src = r#"
            #version 330

            in vec2 v_tex_coords;

            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).expect("Couldn't create a shader program");
        let mut image_path = env::current_dir().expect("Couldn't get current directory");
        image_path.push("assets/images/red.png");
        let image = Self::gen_image_2d(&image_path).expect("Couldn't extract the image");
        let texture = Texture2d::new(&display, image).expect("Couldn't generate texture");

        Self {
            _window: window,
            display,
            vertex_buffer,
            index_buffer,
            program,
            texture,
            is_finished: false,
            update_count: 0,
            render_count: 0
        }
    }

    fn process_input(&mut self, input: WindowEvent) {
        if let WindowEvent::CloseRequested = input {
            self.is_finished = true;
        }
    }

    fn update(&mut self) {
        self.update_count += 1;
    }

    fn fixed_update(&mut self) {
        println!("UPS: {}, FPS: {}", self.update_count, self.render_count);
        self.update_count = 0;
        self.render_count = 0;
    }

    fn render(&mut self) {
        let mut frame = self.display.draw();
        frame.clear_color(0.5, 0.5, 0.5, 1.0);

        frame.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniform! {
                tex: &self.texture
            },
            &glium::DrawParameters::default()
        ).expect("Couldn't draw the triangle");

        frame.finish().expect("Couldn't render the frame");
        self.render_count += 1;
    }

    fn should_close(&self) -> bool {
        self.is_finished
    }
}

impl Game {
    fn gen_image_2d(path: &'_ Path) -> Result<RawImage2d<'_, u8>, ReadImageError> {
        let image_reader = match ImageReader::open(path) {
            Ok(image_reader) => { image_reader }
            Err(err) => { return Err(ReadImageError::IOError(err)); }
        };

        let image = match image_reader.decode() {
            Ok(image) => { image.into_rgba8() }
            Err(err) => { return Err(ReadImageError::ImageError(err)); }
        };

        let (width, height) = image.dimensions();
        let image = RawImage2d::from_raw_rgba(image.into_raw(), (width, height));
        Ok(image)
    }
}

#[derive(Debug)]
enum ReadImageError {
    IOError(io::Error),
    ImageError(image::ImageError)
}

impl fmt::Display for ReadImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReadImageError::ImageError(e) => write!(f, "{}", e),
            ReadImageError::IOError(e) => write!(f, "{}", e)
        }
    }
}

impl std::error::Error for ReadImageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadImageError::ImageError(e) => Some(e),
            ReadImageError::IOError(e) => Some(e)
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, tex_coords);