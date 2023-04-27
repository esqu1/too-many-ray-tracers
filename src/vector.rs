use crate::color::Color;
use std::ops;

#[derive(Clone, Debug, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const ORIGIN: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

impl<'a, 'b> ops::Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn mul(self, other: &'b Vector) -> Vector {
        Vector {
            x: other.x * self.x,
            y: other.y * self.y,
            z: other.z * self.z,
        }
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, other: Vector) -> Self {
        Self {
            x: other.x * self.x,
            y: other.y * self.y,
            z: other.z * self.z,
        }
    }
}

impl<'a> ops::Mul<f64> for &'a Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dot_ref(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

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
        Vector {
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

    pub fn cross(&self, other: &Vector) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn interpolate(&self, t: f64) -> Vector {
        let d = &self.dir * t;
        &d + &self.origin
    }

    pub fn from_pts(v1: Vector, v2: Vector) -> Self {
        // v1 -> v2
        Self {
            origin: v1.clone(),
            dir: v2 - v1,
        }
    }
}
