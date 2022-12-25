#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cgmath;
use cgmath::prelude::*;
use cgmath::vec3;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

#[derive(PartialEq)]
pub enum Move {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    // Camera Attributes
    pub position: Point3,
    pub camera_front: Vector3,
    pub camera_up: Vector3,
    pub camera_right: Vector3,
    pub world_up: Vector3,
    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,
    // Camera options
    pub movement_speed: f32,
    pub sens: f32,
    pub fov: f32,

    pub first_mouse_movement: bool,
    pub last_x: f32,
    pub last_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            camera_front: vec3(0.0, 0.0, -1.0),
            camera_up: Vector3::zero(),
            camera_right: Vector3::zero(),
            world_up: Vector3::unit_y(),
            yaw: -90.0,
            pitch: 0.0,
            movement_speed: 10.0,
            sens: 0.1,
            fov: 45.0,
            first_mouse_movement: true,
            last_x: 400.0,
            last_y: 300.0,
        };
        camera.updateCameraVectors();
        camera
    }

    pub fn GetViewMatrix(&self) -> Matrix4 {
        Matrix4::look_at(
            self.position,
            self.position + self.camera_front,
            self.camera_up,
        )
    }

    pub fn translate(&mut self, camera_move: Move, deltaTime: f32) {
        let velocity = self.movement_speed * deltaTime;
        if camera_move == Move::Forward {
            self.position += self.camera_front * velocity;
        }
        if camera_move == Move::Backward {
            self.position += -(self.camera_front * velocity);
        }
        if camera_move == Move::Left {
            self.position += -(self.camera_right * velocity);
        }
        if camera_move == Move::Right {
            self.position += self.camera_right * velocity;
        }
    }

    pub fn look_around(&mut self, x: f64, y: f64) {
        let (xpos, ypos) = (x as f32, y as f32);
        if self.first_mouse_movement {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse_movement = false;
        }

        let mut xoffset = xpos - self.last_x;
        let mut yoffset = self.last_y - ypos;

        self.last_x = xpos;
        self.last_y = ypos;

        xoffset *= self.sens;
        yoffset *= self.sens;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.updateCameraVectors();
    }

    pub fn zoom(&mut self, yoffset: f32) {
        if self.fov >= 1.0 && self.fov <= 45.0 {
            self.fov -= yoffset;
        }
        if self.fov <= 1.0 {
            self.fov = 1.0;
        }
        if self.fov >= 45.0 {
            self.fov = 45.0;
        }
    }

    fn updateCameraVectors(&mut self) {
        let front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.camera_front = front.normalize();

        self.camera_right = self.camera_front.cross(self.world_up).normalize();
        self.camera_up = self.camera_right.cross(self.camera_front).normalize();
    }
}
