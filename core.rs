use super::matrix::{Matrix, Matrix3D, YMat4};
use super::vector::{YVec2, YVec3};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: YVec3,
    pub uv: YVec2,
    pub normal: YVec3,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub indices: [usize; 3],
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }
}

pub struct Transform {
    pub position: YVec3,
    pub rotation: YVec3, // Euler angles
    pub scale: YVec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: YVec3::new(0.0, 0.0, 0.0),
            rotation: YVec3::new(0.0, 0.0, 0.0),
            scale: YVec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    // W = S * Rx * Ry * Rz * T
    pub fn world_matrix(&self) -> YMat4 {
        let t = YMat4::translation(self.position.x, self.position.y, self.position.z);
        let s = YMat4::scaling(self.scale.x, self.scale.y, self.scale.z);
        let rx = YMat4::rotation_x(self.rotation.x as f64);
        let ry = YMat4::rotation_y(self.rotation.y as f64);
        let rz = YMat4::rotation_z(self.rotation.z as f64);

        let mut r = s.multiply(&rx);
        r = r.multiply(&ry);
        r = r.multiply(&rz);
        r = r.multiply(&t);
        r
    }
}
