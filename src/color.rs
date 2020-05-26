use smth::{Float, Number};
use std::ops::{Add, AddAssign, Mul};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Color<S> {
    pub r: S,
    pub g: S,
    pub b: S,
}

impl<S> Color<S> {    
    pub const fn new(r: S, g: S, b: S) -> Self {
        Self { r, g, b }
    }
}

impl<S: Float> Color<S> {
    pub const BLACK: Self = Self::new(S::ZERO, S::ZERO, S::ZERO);
    pub const WHITE: Self = Self::new(S::ONE, S::ONE, S::ONE);
    
    pub fn lerp(&self, other: &Self, t: S) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }
}

impl<S> Into<[S; 3]> for Color<S> {
    fn into(self) -> [S; 3] {
        [self.r, self.g, self.b]
    }
}

impl<S: Copy> From<[S; 3]> for Color<S> {
    fn from(array: [S; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }
}

impl<S: Number> Add for Color<S> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl<S: Number> AddAssign for Color<S> {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl<S: Number> Mul<S> for Color<S> {
    type Output = Self;
    
    fn mul(self, other: S) -> Self::Output {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }   
}

impl<S: Number> Mul for Color<S> {
    type Output = Self;
    
    fn mul(self, other: Color<S>) -> Self::Output {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }   
}
