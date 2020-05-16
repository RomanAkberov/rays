use std::ops::{Add, Mul};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(into = "[f64; 3]")]
#[serde(from = "[f64; 3]")]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }
}

impl Into<[f64; 3]> for Color {
    fn into(self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<[f64; 3]> for Color {
    fn from(array: [f64; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    
    fn mul(self, other: f64) -> Self::Output {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }   
}

impl Mul for Color {
    type Output = Self;
    
    fn mul(self, other: Color) -> Self::Output {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }   
}
