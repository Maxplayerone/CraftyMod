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
    //overall
    matrix: math::Mat4,
    location: i32,

    camera_pos: math::Vec3,
    camera_front: math::Vec3,
    camera_up: math::Vec3,
    target: math::Vec3,

    //translation
    movement_speed: f32,
    //look around
    first_time_flag: bool,
    last_x: f32,
    last_y: f32,
    pitch: f32,
    yaw: f32,
    sens: f32,
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
            movement_speed: 10.0,
                first_time_flag: true,
                last_x: 400.0, //probably should somehow get the dimensions of the window and not hardcode it
                last_y: 300.0,
                pitch: 0.0,
                yaw: -90.0,
                sens: 0.1,
        }
    }

    pub fn update_camera_position(&mut self){
        self.target = &self.camera_pos + &self.camera_front;
        self.matrix.look_at(&self.camera_pos, &self.target, &self.camera_up);
        unsafe{
        gl::UniformMatrix4fv(self.location, 1, gl::FALSE, &self.matrix.mat[0]);
        }
    }
    pub fn translate(&mut self, move_type: Move, delta_time: f64) {
        let movement_speed_cached = self.movement_speed;
        self.movement_speed *= delta_time as f32;

        match move_type {
            Move::Up => {
                self.camera_pos =
                    &self.camera_pos + &math::Vec3::mul(&self.camera_front, self.movement_speed);
            }
            Move::Down => {
                self.camera_pos =
                    &self.camera_pos - &math::Vec3::mul(&self.camera_front, self.movement_speed);
            }
            Move::Left => {
                self.camera_pos = &self.camera_pos
                    - &math::Vec3::mul(
                        &(math::Vec3::cross(&self.camera_front, &self.camera_up).normalize()),
                        self.movement_speed,
                    );
            },
            Move::Right => {
                self.camera_pos = &self.camera_pos
                    + &math::Vec3::mul(
                        &(math::Vec3::cross(&self.camera_front, &self.camera_up).normalize()),
                        self.movement_speed,
                    );
            }
        }

        self.movement_speed = movement_speed_cached;
    }

    pub fn look_around(&mut self, x: f64, y: f64){

        if self.first_time_flag{
            self.last_x = x as f32;
            self.last_y = y as f32;
            self.first_time_flag = false;
        }
        let mut x_offset = self.last_x - x as f32;
        let mut y_offset = y as f32 - self.last_y;
        
        self.last_x = x as f32;
        self.last_y = y as f32;

        x_offset *= self.sens;
        y_offset *= self.sens;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if self.pitch > 89.0{
            self.pitch = 89.0;
        }
        if self.pitch < -89.0{
            self.pitch = -89.0;
        }
        
        let mut dir = math::Vec3::new(0.0, 0.0, 0.0);
        dir.x = (math::rad(self.yaw)).cos() * (math::rad(self.pitch)).cos();
        dir.y = (math::rad(self.pitch)).sin();
        dir.z = (math::rad(self.yaw)).sin() * (math::rad(self.pitch)).cos();
        self.camera_front = dir.normalize();
    } 
}
