use super::matrix::{Matrix3D, YMat4};
use super::vector::YVec3;

pub struct Camera {
    pub position: YVec3,
    pub target: YVec3,
    pub up: YVec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: YVec3, target: YVec3, aspect: f32, fov: Option<f32>) -> Self {
        Self {
            position,
            target,
            up: YVec3::new(0.0, 1.0, 0.0),
            fov: fov.unwrap_or(std::f32::consts::PI / 3.0),
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> YMat4 {
        YMat4::look_at(
            [self.position.x, self.position.y, self.position.z],
            [self.target.x, self.target.y, self.target.z],
            [self.up.x, self.up.y, self.up.z],
        )
    }

    pub fn projection_matrix(&self) -> YMat4 {
        YMat4::perspective(self.fov, self.aspect, self.near, self.far)
    }

    /// Update the camera's position to orbit around its target.
    /// `radius`: Horizontal distance from the target.
    /// `angle`: Orbit angle in radians.
    /// `height`: Vertical offset from the target.
    pub fn orbit(&mut self, radius: f32, angle: f32, height: f32) {
        self.position.x = self.target.x + radius * angle.sin();
        self.position.z = self.target.z - radius * angle.cos();
        self.position.y = self.target.y + height;
    }
}
