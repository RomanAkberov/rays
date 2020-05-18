use std::ops::{Add, AddAssign, Mul};
use serde::{Serialize, Deserialize};
use crate::math::Float;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Color {    
    pub const fn new(r: Float, g: Float, b: Float) -> Self {
        Self { r, g, b }
    }
}

impl Color {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    
    pub fn lerp(&self, other: &Self, t: Float) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }
}

impl Into<[Float; 3]> for Color {
    fn into(self) -> [Float; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<[Float; 3]> for Color {
    fn from(array: [Float; 3]) -> Self {
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

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Mul<Float> for Color {
    type Output = Self;
    
    fn mul(self, other: Float) -> Self::Output {
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
