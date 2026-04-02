use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YMat4 {
    pub x: [f32; 4],
    pub y: [f32; 4],
    pub z: [f32; 4],
    pub w: [f32; 4],
}

impl YMat4 {
    pub fn transform_vec3(
        &self,
        v: &crate::three_rs::vector::YVec3,
    ) -> crate::three_rs::vector::YVec3 {
        let x = v.x * self.x[0] + v.y * self.y[0] + v.z * self.z[0] + 1.0 * self.w[0];
        let y = v.x * self.x[1] + v.y * self.y[1] + v.z * self.z[1] + 1.0 * self.w[1];
        let z = v.x * self.x[2] + v.y * self.y[2] + v.z * self.z[2] + 1.0 * self.w[2];
        let w = v.x * self.x[3] + v.y * self.y[3] + v.z * self.z[3] + 1.0 * self.w[3];
        if w != 0.0 && w != 1.0 {
            crate::three_rs::vector::YVec3::new(x / w, y / w, z / w)
        } else {
            crate::three_rs::vector::YVec3::new(x, y, z)
        }
    }
}

impl Mul<crate::three_rs::vector::YVec3> for YMat4 {
    type Output = crate::three_rs::vector::YVec3;
    fn mul(self, rhs: crate::three_rs::vector::YVec3) -> Self::Output {
        self.transform_vec3(&rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YMat3 {
    pub x: [f32; 3],
    pub y: [f32; 3],
    pub z: [f32; 3],
}

/// Core trait for any M x N matrix.
pub trait Matrix<T>: Sized
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    /// Create a new matrix filled with a default value.
    fn new(rows: usize, cols: usize) -> Self;

    /// Create an identity matrix (must be square).
    fn identity(size: usize) -> Self;

    /// Returns the dimensions as (rows, columns).
    fn dimensions(&self) -> (usize, usize);

    /// Access an element at (row, col) without bounds checking (unsafe).
    fn get_unchecked(&self, row: usize, col: usize) -> T;

    /// Set an element at (row, col).
    fn set(&mut self, row: usize, col: usize, value: T);

    /// Transpose the matrix.
    fn transpose(&self) -> Self;

    /// Standard Matrix Multiplication (Dot Product).
    fn multiply(&self, other: &Self) -> Self;

    /// Scalar multiplication.
    fn scale(&mut self, factor: T);
}

/// Extension for 3D-specific transformations (typically 4x4).
pub trait Matrix3D<T>: Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    /// Create a translation matrix.
    fn translation(x: T, y: T, z: T) -> Self;

    /// Create a scaling matrix.
    fn scaling(x: T, y: T, z: T) -> Self;

    /// Create a rotation matrix around the X axis (radians).
    fn rotation_x(angle: f64) -> Self;

    /// Create a rotation matrix around the Y axis (radians).
    fn rotation_y(angle: f64) -> Self;

    /// Create a rotation matrix around the Z axis (radians).
    fn rotation_z(angle: f64) -> Self;

    /// LookAt matrix for camera views.
    fn look_at(eye: [T; 3], center: [T; 3], up: [T; 3]) -> Self;

    /// Perspective projection matrix.
    fn perspective(fov_rad: T, aspect: T, near: T, far: T) -> Self;
}

impl Matrix<f32> for YMat4 {
    fn new(rows: usize, cols: usize) -> Self {
        assert_eq!(rows, 4, "YMat4 must have 4 rows");
        assert_eq!(cols, 4, "YMat4 must have 4 columns");
        YMat4 {
            x: [0.0; 4],
            y: [0.0; 4],
            z: [0.0; 4],
            w: [0.0; 4],
        }
    }

    fn identity(size: usize) -> Self {
        assert_eq!(size, 4, "YMat4 identity must be size 4");
        YMat4 {
            x: [1.0, 0.0, 0.0, 0.0],
            y: [0.0, 1.0, 0.0, 0.0],
            z: [0.0, 0.0, 1.0, 0.0],
            w: [0.0, 0.0, 0.0, 1.0],
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (4, 4)
    }

    fn get_unchecked(&self, row: usize, col: usize) -> f32 {
        let r = match row {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Row out of bounds"),
        };
        r[col]
    }

    fn set(&mut self, row: usize, col: usize, value: f32) {
        let r = match row {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Row out of bounds"),
        };
        r[col] = value;
    }

    fn transpose(&self) -> Self {
        let mut result = Self::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                result.set(j, i, self.get_unchecked(i, j));
            }
        }
        result
    }

    fn multiply(&self, other: &Self) -> Self {
        let mut result = Self::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.get_unchecked(i, k) * other.get_unchecked(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    fn scale(&mut self, factor: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.set(i, j, self.get_unchecked(i, j) * factor);
            }
        }
    }
}

impl Matrix3D<f32> for YMat4 {
    fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut r = Self::identity(4);
        r.set(3, 0, x);
        r.set(3, 1, y);
        r.set(3, 2, z);
        r
    }

    fn scaling(x: f32, y: f32, z: f32) -> Self {
        let mut r = Self::identity(4);
        r.set(0, 0, x);
        r.set(1, 1, y);
        r.set(2, 2, z);
        r
    }

    fn rotation_x(angle: f64) -> Self {
        let mut r = Self::identity(4);
        let c = angle.cos() as f32;
        let s = angle.sin() as f32;
        r.set(1, 1, c);
        r.set(1, 2, s);
        r.set(2, 1, -s);
        r.set(2, 2, c);
        r
    }

    fn rotation_y(angle: f64) -> Self {
        let mut r = Self::identity(4);
        let c = angle.cos() as f32;
        let s = angle.sin() as f32;
        r.set(0, 0, c);
        r.set(0, 2, -s);
        r.set(2, 0, s);
        r.set(2, 2, c);
        r
    }

    fn rotation_z(angle: f64) -> Self {
        let mut r = Self::identity(4);
        let c = angle.cos() as f32;
        let s = angle.sin() as f32;
        r.set(0, 0, c);
        r.set(0, 1, s);
        r.set(1, 0, -s);
        r.set(1, 1, c);
        r
    }

    fn look_at(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> Self {
        let mut f = [center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]]; // Forward vector
        let f_len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt(); // Normalize forward vector
        f[0] /= f_len;
        f[1] /= f_len;
        f[2] /= f_len;

        let mut s = [
            f[1] * up[2] - f[2] * up[1],
            f[2] * up[0] - f[0] * up[2],
            f[0] * up[1] - f[1] * up[0],
        ]; // Side vector
        let s_len = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt(); // Normalize side vector
        s[0] /= s_len;
        s[1] /= s_len;
        s[2] /= s_len;

        let u = [
            // Up vector
            s[1] * f[2] - s[2] * f[1],
            s[2] * f[0] - s[0] * f[2],
            s[0] * f[1] - s[1] * f[0],
        ];

        let mut r = Self::identity(4);
        r.set(0, 0, s[0]);
        r.set(0, 1, u[0]);
        r.set(0, 2, -f[0]);

        r.set(1, 0, s[1]);
        r.set(1, 1, u[1]);
        r.set(1, 2, -f[1]);

        r.set(2, 0, s[2]);
        r.set(2, 1, u[2]);
        r.set(2, 2, -f[2]);

        r.set(3, 0, -(s[0] * eye[0] + s[1] * eye[1] + s[2] * eye[2])); // Translation vector
        r.set(3, 1, -(u[0] * eye[0] + u[1] * eye[1] + u[2] * eye[2])); // Translation vector
        r.set(3, 2, f[0] * eye[0] + f[1] * eye[1] + f[2] * eye[2]); // Translation vector

        r
    }

    fn perspective(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut r = Self::new(4, 4);
        let f = 1.0 / (fov_rad / 2.0).tan();
        r.set(0, 0, f / aspect);
        r.set(1, 1, f);
        r.set(2, 2, far / (near - far));
        r.set(2, 3, -1.0);
        r.set(3, 2, (near * far) / (near - far));
        r.set(3, 3, 0.0);
        r
    }
}
