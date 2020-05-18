use std::ops::{Add, AddAssign, Mul};
use serde::{Serialize, Deserialize};
use crate::math::Float;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Color<F> {
    pub r: F,
    pub g: F,
    pub b: F,
}

impl<F> Color<F> {    
    pub const fn new(r: F, g: F, b: F) -> Self {
        Self { r, g, b }
    }
}

impl<F: Float> Color<F> {
    pub const BLACK: Self = Self::new(F::ZERO, F::ZERO, F::ZERO);
    pub const WHITE: Self = Self::new(F::ONE, F::ONE, F::ONE);
    
    pub fn lerp(&self, other: &Self, t: F) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }
}

impl<F: Float> Into<[F; 3]> for Color<F> {
    fn into(self) -> [F; 3] {
        [self.r, self.g, self.b]
    }
}

impl<F: Float> From<[F; 3]> for Color<F> {
    fn from(array: [F; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }
}

impl<F: Float> Add for Color<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl<F: Float> AddAssign for Color<F> {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl<F: Float> Mul<F> for Color<F> {
    type Output = Self;
    
    fn mul(self, other: F) -> Self::Output {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }   
}

impl<F: Float> Mul for Color<F> {
    type Output = Self;
    
    fn mul(self, other: Color<F>) -> Self::Output {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }   
}
