use std::ops::{Add, Sub, Neg, Mul};
use super::Float;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: Float,
    pub y: Float,
}

impl Vec2 {
    pub const fn new(x: Float, y: Float) -> Self {
        Self { x, y, }
    }
}

impl Vec2 {
    pub fn len(self) -> Float {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn len2(self) -> Float {
        self.x * self.x + self.y * self.y
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<Float> for Vec2 {
    type Output = Self;

    fn mul(self, other: Float) -> Self::Output {
        Self::new(self.x * other, self.y * other)
    }
}
