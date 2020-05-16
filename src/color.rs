use std::ops::Mul;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Deserialize)]
#[serde(from = "[f64; 3]")]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
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

impl From<[f64; 3]> for Color {
    fn from(array: [f64; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
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
