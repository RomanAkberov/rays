use std::ops::*;
use super::Float;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2(pub [Float; 2]);

impl Vec2 {
    pub fn len(self) -> Float {
        (self[0] * self[0] + self[1] * self[1]).sqrt()
    }

    pub fn len2(self) -> Float {
        self[0] * self[0] + self[1] * self[1]
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self([
            self[0] + other[0],
            self[1] + other[1],
        ])
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self([
            self[0] - other[0], 
            self[1] - other[1],
        ])
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self([
            -self[0], 
            -self[1],
        ])
    }
}

impl Mul<Float> for Vec2 {
    type Output = Self;

    fn mul(self, other: Float) -> Self::Output {
        Self([
            self[0] * other, 
            self[1] * other,
        ])
    }
}

impl Deref for Vec2 {
    type Target = [Float; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}