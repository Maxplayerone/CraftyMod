use crate::renderer::buffer::Buffer;
use crate::renderer::camera::Camera;
use crate::renderer::camera::Move;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::texture::Texture;
use crate::renderer::vertex_array::VertexArray;

use std::ffi::CStr;
use std::path::Path;

use crate::utils::math;

//convert literals to c strings without any runtime overhead
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

pub struct Renderer {
    program: ShaderProgram,
    vbo: Buffer,
    ibo: Buffer,
    vao: VertexArray,
    tex0: Texture,
    tex1: Texture,
    camera: Camera,
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            let vertex_shader = Shader::new("src/shaders/basic.vs", gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new("src/shaders/basic.frag", gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let element_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            let vertex_array = VertexArray::new();

            let tex0 = Texture::new(Path::new("src/resources/container.jpg"));
            let tex1 = Texture::new(Path::new("src/resources/awesomeface.png"));

            program.set_int(c_str!("tex0"), 0);
            program.set_int(c_str!("tex1"), 1);

            let camera = Camera::new(&program);

            let mut model = math::Mat4::new(1.0);
            model.rotate(math::Vec3::new(0.5, 1.0, 0.0).normalize(), 32.0);

            let model_loc = gl::GetUniformLocation(program.id, c_str!("model").as_ptr());
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model.mat[0]);

            let mut model = math::Mat4::new(1.0);
            model.rotate(math::Vec3::new(0.5, 1.0, 0.0).normalize(), 32.0);

            Ok(Self {
                program,
                vbo: vertex_buffer,
                ibo: element_buffer,
                vao: vertex_array,
                tex0,
                tex1,
                camera,
            })
        }
    }

    pub fn upload_vbo_data(&self, data: &[f32]) {
        unsafe {
            self.vbo.set_data(data, gl::STATIC_DRAW);
        }
    }

    pub fn upload_ibo_data(&self, data: &[u32]) {
        unsafe {
            self.vao.bind();
            self.ibo.set_data(data, gl::STATIC_DRAW);
        }
    }

    pub fn set_vao_attrib(
        &self,
        loc: u32,
        num_of_components: i32,
        vertex_size: usize,
        offset: usize,
    ) {
        unsafe {
            self.vao.bind();
            self.vbo.bind();
            self.vao
                .set_attribute(loc, num_of_components, vertex_size, offset);
        }
    }

    pub fn process_events(&mut self, event: glfw::WindowEvent, delta_time: f64) {
        match event {
            glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Repeat, _) => {
                self.camera.translate(Move::Up, delta_time)
            }
            glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Repeat, _) => {
                self.camera.translate(Move::Down, delta_time)
            }
            glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Repeat, _) => {
                self.camera.translate(Move::Left, delta_time)
            }
            glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Repeat, _) => {
                self.camera.translate(Move::Right, delta_time)
            }
            glfw::WindowEvent::CursorPos(x, y) => self.camera.look_around(x, y),
            glfw::WindowEvent::Scroll(_x, y) => self.camera.zoom(y),
            glfw::WindowEvent::MouseButton(
                glfw::MouseButton::Button1,
                glfw::Action::Release,
                _,
            ) => self.camera.change_looking_around_state(),
            _ => (),
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            self.tex0.bind();
            gl::ActiveTexture(gl::TEXTURE1);
            self.tex1.bind();
            self.program.bind();

            self.camera.update_camera_position();
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, std::ptr::null());
            //gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
