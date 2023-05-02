use crate::color::Color;
use num_traits::Num;
use std::ops;

#[derive(Clone, Debug, Default)]

pub struct Vector<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vector<f64>;
pub type Vec3i = Vector<i32>;

pub const ORIGIN: Vec3f = Vec3f {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

impl<'a, 'b> ops::Add<&'b Vec3f> for &'a Vec3f {
    type Output = Vec3f;

    fn add(self, other: &'b Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Add for Vec3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vec3f> for &'a Vec3f {
    type Output = Vec3f;

    fn sub(self, other: &'b Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Sub for Vec3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Vec3f> for &'a Vec3f {
    type Output = Vec3f;

    fn mul(self, other: &'b Vec3f) -> Vec3f {
        Vec3f {
            x: other.x * self.x,
            y: other.y * self.y,
            z: other.z * self.z,
        }
    }
}

impl ops::Mul<Vec3f> for Vec3f {
    type Output = Self;

    fn mul(self, other: Vec3f) -> Self {
        Self {
            x: other.x * self.x,
            y: other.y * self.y,
            z: other.z * self.z,
        }
    }
}

impl<'a> ops::Mul<f64> for &'a Vec3f {
    type Output = Vec3f;

    fn mul(self, other: f64) -> Vec3f {
        Vec3f {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl ops::Mul<f64> for Vec3f {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl<T: Num + Copy> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Vector<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dot_ref(&self, other: &Vector<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3f {
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn sq_norm(&self) -> f64 {
        self.norm() * self.norm()
    }

    pub fn normalize(&self) -> Self {
        self * (1.0 / self.norm())
    }

    pub fn from_color(color: Color) -> Self {
        Vec3f {
            x: color.red as f64 / 255.0,
            y: color.green as f64 / 255.0,
            z: color.blue as f64 / 255.0,
        }
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn cross(&self, other: &Vec3f) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub dir: Vec3f,
}

impl Ray {
    pub fn interpolate(&self, t: f64) -> Vec3f {
        let d = &self.dir * t;
        &d + &self.origin
    }

    pub fn from_pts(v1: Vec3f, v2: Vec3f) -> Self {
        // v1 -> v2
        Self {
            origin: v1.clone(),
            dir: v2 - v1,
        }
    }
}
