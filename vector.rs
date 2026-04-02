use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YVec2 {
    pub x: f32,
    pub y: f32,
}

impl YVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl AddAssign for YVec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl YVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            self
        }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*other - *self).length()
    }

    pub fn lerp(v0: Self, v1: Self, t: f32) -> Self {
        v0 + (v1 - v0) * t
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2.0 * self.dot(normal)
    }

    pub fn mul_vec(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Add for YVec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for YVec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for YVec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for YVec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl AddAssign for YVec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Support &YVec3 operators
impl Add for &YVec3 { type Output = YVec3; fn add(self, other: Self) -> YVec3 { *self + *other } }
impl Sub for &YVec3 { type Output = YVec3; fn sub(self, other: Self) -> YVec3 { *self - *other } }
impl Mul<f32> for &YVec3 { type Output = YVec3; fn mul(self, scalar: f32) -> YVec3 { *self * scalar } }
impl Div<f32> for &YVec3 { type Output = YVec3; fn div(self, scalar: f32) -> YVec3 { *self / scalar } }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl YVec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<YVec3> for YVec4 {
    fn from(v: YVec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 1.0,
        }
    }
}

impl Add for YVec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Sub for YVec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl Mul<f32> for YVec4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl AddAssign for YVec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}
