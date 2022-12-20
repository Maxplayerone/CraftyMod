use crate::utils::math;

extern crate cgmath;

use crate::renderer::program::ShaderProgram;

use std::ffi::CStr;
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

pub struct Camera {
    matrix: math::Mat4,
    location: i32,

    camera_pos: math::Vec3,
    camera_front: math::Vec3,
    camera_up: math::Vec3,
    target: math::Vec3,

    movement_speed: f32,
}

impl Camera {
    pub unsafe fn new(program: &ShaderProgram) -> Self {
        let camera_pos = math::Vec3::new(0.0, 0.0, 3.0);
        let camera_front = math::Vec3::new(0.0, 0.0, -1.0);
        let camera_up = math::Vec3::new(0.0, 1.0, 0.0);
        let target = &camera_pos + &camera_front;

        let mut view = math::Mat4::new(1.0);
        view.look_at(&camera_pos, &target, &camera_up);

        let view_loc = gl::GetUniformLocation(program.id, c_str!("view").as_ptr());
        gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view.mat[0]);

        Self {
            matrix: view,
            location: view_loc,
            camera_pos: camera_pos,
            camera_front: camera_front,
            camera_up: camera_up,
            target: target,
            movement_speed: 0.1,
        }
    }
    pub fn translate(&mut self, move_type: Move) {
        match move_type {
            Move::Up => {
                self.camera_pos = &self.camera_pos + &math::Vec3::mul(&self.camera_front, self.movement_speed);
                self.target = &self.camera_pos + &self.camera_front;
                self.matrix.look_at(&self.camera_pos, &self.target, &self.camera_up);
                unsafe{
                gl::UniformMatrix4fv(self.location, 1, gl::FALSE, &self.matrix.mat[0]);
                }
            },
            Move::Down => {
                self.camera_pos = &self.camera_pos - &math::Vec3::mul(&self.camera_front, self.movement_speed);
                self.target = &self.camera_pos + &self.camera_front;
                self.matrix.look_at(&self.camera_pos, &self.target, &self.camera_up);
                unsafe{
                gl::UniformMatrix4fv(self.location, 1, gl::FALSE, &self.matrix.mat[0]);
                }
            },
            Move::Left => {
                self.camera_pos = &self.camera_pos
                    + &math::Vec3::mul(
                        &(math::Vec3::cross(&self.camera_front, &self.camera_up).normalize()),
                        self.movement_speed,
                    );
                self.target = &self.camera_pos + &self.camera_front;
                self.matrix.look_at(&self.camera_pos, &self.target, &self.camera_up);
                unsafe{
                gl::UniformMatrix4fv(self.location, 1, gl::FALSE, &self.matrix.mat[0]);
                }
            },
            Move::Right => {
                self.camera_pos = &self.camera_pos
                    - &math::Vec3::mul(
                        &(math::Vec3::cross(&self.camera_front, &self.camera_up).normalize()),
                        self.movement_speed,
                    );
                self.target = &self.camera_pos + &self.camera_front;
                self.matrix.look_at(&self.camera_pos, &self.target, &self.camera_up);
                unsafe{
                gl::UniformMatrix4fv(self.location, 1, gl::FALSE, &self.matrix.mat[0]);
                }
            }
        }
    }
}
