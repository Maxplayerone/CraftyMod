use crate::renderer::buffer::Buffer;
use crate::renderer::camera::Camera;
use crate::renderer::camera::Move;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::texture::Texture;
use crate::renderer::vertex_array::VertexArray;
use crate::renderer::vertex_array::VertexArrayConfiguration;

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
    tex: Texture,
    camera: Camera,
    cube_count: i32,
    cube_size: f32,
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

            let tex = Texture::new(Path::new("src/resources/test_grass.png"));

            program.set_int(c_str!("tex0"), 0);

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
                tex,
                camera,
                cube_count: 0,
                cube_size: 1.0,
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

    pub fn process_input(&mut self, window: &glfw::Window, delta_time: f64) {
        if glfw::Window::get_key(window, glfw::Key::W) == glfw::Action::Press {
            self.camera.translate(Move::Up, delta_time);
        }
        if glfw::Window::get_key(window, glfw::Key::S) == glfw::Action::Press {
            self.camera.translate(Move::Down, delta_time);
        }
        if glfw::Window::get_key(window, glfw::Key::A) == glfw::Action::Press {
            self.camera.translate(Move::Left, delta_time);
        }
        if glfw::Window::get_key(window, glfw::Key::D) == glfw::Action::Press {
            self.camera.translate(Move::Right, delta_time);
        }
    }

    pub fn process_events(&mut self, event: glfw::WindowEvent, delta_time: f64) {
        match event {
            glfw::WindowEvent::CursorPos(x, y) => self.camera.look_around(x, y),
            glfw::WindowEvent::Scroll(_x, y) => self.camera.zoom(y),
            _ => (),
        }
    }

    pub fn load_cubes(&mut self, starting_pos: math::Vec3, dimensions: math::Vec3) {
        let width = dimensions.x as i32;
        let height = dimensions.y as i32;
        let depth = dimensions.z as i32;
        self.cube_count = width * height * depth;

        let mut offset_x = 0.0;
        let mut offset_y = 0.0;
        let mut offset_z = 0.0;
        let mut i = 0;

        let mut vertices: Vec<f32> = Vec::with_capacity((120 * self.cube_count) as usize);
        let size = self.cube_size;
        for _ in 0..self.cube_count {
            println!("offset x {}", offset_x);
            println!("offset y {}", offset_y);
            println!("offset z {}", offset_z);
            println!("----------------------------");
            //back-face
            vertices.push(starting_pos.x + offset_x); //left-bottom-back
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(0.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + size + offset_x); //right-bottom-back
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(1.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + size + offset_x); //right-top-back
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(1.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + offset_x); //left-top-back
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(0.0);
            vertices.push(1.0);
            //front face
            vertices.push(starting_pos.x + offset_x); //left-bottom-front
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(0.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + size + offset_x); //right-bottom-front
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(1.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + size + offset_x); //right-top-front
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(1.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + offset_x); //left-top-front
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(0.0);
            vertices.push(1.0);
            //left face
            vertices.push(starting_pos.x + offset_x); //left-top-front
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(1.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + offset_x); //left-top-back
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(1.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + offset_x); //left-bottom-back
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(0.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + offset_x); //left-bottom-front
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(0.0);
            vertices.push(0.0);
            //right face
            vertices.push(starting_pos.x + size + offset_x); //right-top-front
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(1.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + size + offset_x); //right-top-back
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(1.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + size + offset_x); //right-bottom-back
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(0.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + size + offset_x); //right-bottom-front
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(0.0);
            vertices.push(0.0);
            //bottom face
            vertices.push(starting_pos.x + offset_x); //left-bottom-back
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(0.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + size + offset_x); //right-bottom-back
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(1.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + size + offset_x); //right-bottom-front
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(1.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + offset_x); //left-bottom-fronts
            vertices.push(starting_pos.y + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(0.0);
            vertices.push(0.0);
            //top face
            vertices.push(starting_pos.x + offset_x); //left-top-back
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(0.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + size + offset_x); //right-top-back
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + offset_z);
            vertices.push(1.0);
            vertices.push(1.0);
            vertices.push(starting_pos.x + size + offset_x); //right-top-front
            vertices.push(starting_pos.y + size + offset_y);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(1.0);
            vertices.push(0.0);
            vertices.push(starting_pos.x + offset_x); //left-top-front
            vertices.push(starting_pos.y + size);
            vertices.push(starting_pos.z + size + offset_z);
            vertices.push(0.0);
            vertices.push(0.0);

            i += 1;
            offset_x += 1.0;
            if i % width == 0 {
                offset_x = 0.0;
                offset_z += 1.0;
            }
            /*
            if i % width * height == 0 {
                offset_x = 0.0;
                offset_y = 0.0;
                offset_y += 1.0;
            }
            */
        }

        let mut indices: Vec<u32> = Vec::with_capacity((36 * self.cube_count) as usize);
        let mut offset = 0;
        for _ in 0..self.cube_count {
            indices.push(0 + offset);
            indices.push(1 + offset);
            indices.push(2 + offset);
            indices.push(2 + offset);
            indices.push(3 + offset);
            indices.push(0 + offset);
            indices.push(4 + offset);
            indices.push(5 + offset);
            indices.push(6 + offset);
            indices.push(6 + offset);
            indices.push(7 + offset);
            indices.push(4 + offset);
            indices.push(8 + offset);
            indices.push(9 + offset);
            indices.push(10 + offset);
            indices.push(10 + offset);
            indices.push(11 + offset);
            indices.push(8 + offset);
            indices.push(12 + offset);
            indices.push(13 + offset);
            indices.push(14 + offset);
            indices.push(14 + offset);
            indices.push(15 + offset);
            indices.push(12 + offset);
            indices.push(16 + offset);
            indices.push(17 + offset);
            indices.push(18 + offset);
            indices.push(18 + offset);
            indices.push(19 + offset);
            indices.push(16 + offset);
            indices.push(20 + offset);
            indices.push(21 + offset);
            indices.push(22 + offset);
            indices.push(22 + offset);
            indices.push(23 + offset);
            indices.push(20 + offset);

            offset += 24;
        }

        self.upload_vbo_data(&vertices);
        self.upload_ibo_data(&indices);

        self.vao
            .setup_vao(VertexArrayConfiguration::XyzAndTexCoords);
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            self.tex.bind();
            self.program.bind();

            self.camera.update_camera_position();
            self.vao.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                36 * self.cube_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            //gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
