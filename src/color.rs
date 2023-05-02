use crate::vector::Vec3f;
use std::fmt;
use std::ops;

// Implements a RGB color. Values range from 0 to 255.
#[derive(PartialEq, Clone, Default, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn from_vec(v: Vec3f) -> Self {
        Self::new(
            (v.x.abs() * 255.0) as u8,
            (v.y.abs() * 255.0) as u8,
            (v.z.abs() * 255.0) as u8,
        )
    }

    pub fn from_vec_255(v: Vec3f) -> Self {
        Self::new(
            v.x.clamp(0.0, 255.0) as u8,
            v.y.clamp(0.0, 255.0) as u8,
            v.z.clamp(0.0, 255.0) as u8,
        )
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn random() -> Self {
        Self::new(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        )
    }
}

impl<'a, 'b> ops::Add<&'b Color> for &'a Color {
    type Output = Color;

    fn add(self, other: &'b Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Color> for &'a Color {
    type Output = Color;

    fn sub(self, other: &'b Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl<'a> ops::Mul<f64> for &'a Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            red: (other * self.red as f64) as u8,
            green: (other * self.green as f64) as u8,
            blue: (other * self.blue as f64) as u8,
        }
    }
}

impl<'a> ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            red: (other * self.red as f64) as u8,
            green: (other * self.green as f64) as u8,
            blue: (other * self.blue as f64) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.red, self.green, self.blue)
    }
}
