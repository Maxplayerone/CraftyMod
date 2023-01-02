use crate::renderer::camera::Camera;
use crate::renderer::camera::Move;
use crate::renderer::chunk::Chunk;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::texture::Texture;

use std::ffi::CStr;
use std::path::Path;

use crate::utils::math;

use cgmath::{perspective, Deg, Matrix4};

//convert literals to c strings without any runtime overhead
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

pub struct Renderer {
    program: ShaderProgram,
    tex: Texture,
    camera: Camera,
    chunk: Chunk,
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            let vertex_shader = Shader::new("src/shaders/basic.vs", gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new("src/shaders/basic.frag", gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let tex = Texture::new(Path::new("src/resources/stone.png"));

            program.set_int(c_str!("tex0"), 0);

            let camera = Camera::new(math::Vec3::new(0.0, 1.0, 5.0));

            let mut model = math::Mat4::new(1.0);
            model.rotate(math::Vec3::new(0.5, 1.0, 0.0).normalize(), 32.0);

            let model_loc = gl::GetUniformLocation(program.id, c_str!("model").as_ptr());
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model.mat[0]);

            let mut model = math::Mat4::new(1.0);
            model.rotate(math::Vec3::new(0.5, 1.0, 0.0).normalize(), 32.0);

            let mut chunk = Chunk::new();
            chunk.load_cubes(math::Vec3::new(0.0, 0.0, 0.0));

            Ok(Self {
                program,
                tex,
                camera,
                chunk,
            })
        }
    }

    pub fn process_input(&mut self, window: &glfw::Window, delta_time: f64) {
        if glfw::Window::get_key(window, glfw::Key::W) == glfw::Action::Press {
            self.camera.translate(Move::Forward, delta_time as f32);
        }
        if glfw::Window::get_key(window, glfw::Key::S) == glfw::Action::Press {
            self.camera.translate(Move::Backward, delta_time as f32);
        }
        if glfw::Window::get_key(window, glfw::Key::A) == glfw::Action::Press {
            self.camera.translate(Move::Left, delta_time as f32);
        }
        if glfw::Window::get_key(window, glfw::Key::D) == glfw::Action::Press {
            self.camera.translate(Move::Right, delta_time as f32);
        }
    }

    pub fn process_events(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::CursorPos(x, y) => self.camera.look_around(x, y),
            glfw::WindowEvent::Scroll(_x, y) => self.camera.zoom(y as f32),
            _ => (),
        }
    }

    pub fn clear_screen(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.tex.bind();
            self.program.bind();

            let projection: Matrix4<f32> =
                perspective(Deg(self.camera.fov), (800.0 / 600.0) as f32, 0.1, 100.0);
            self.program.set_mat4(c_str!("projection"), &projection);

            // camera/view transformation
            let view = self.camera.get_view_matrix();
            self.program.set_mat4(c_str!("view"), &view);

            self.chunk.render();
        }
    }
}
