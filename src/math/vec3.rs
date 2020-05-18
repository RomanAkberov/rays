use std::ops::*;
use serde::Deserialize;
use super::Float;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Deserialize)]
pub struct Vec3(pub [Float; 3]);

impl Vec3 {
    pub const fn splat(v: Float) -> Self {
        Self([v, v, v])
    }

    pub const ZERO: Self = Self([0.0, 0.0, 0.0]);

    pub fn normalized(self) -> Self {
        let scale = 1.0 / self.len();
        Self([
            self[0] * scale, 
            self[1] * scale, 
            self[2] * scale,
        ])
    }

    pub fn len(self) -> Float {
        (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt()
    }

    pub fn len2(self) -> Float {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn distance(self, other: Self) -> Float {
        (self - other).len()
    }

    pub fn dot(self, other: Self) -> Float {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(self, other: Self) -> Self {
        Self([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }

    pub fn min(self, other: Self) -> Self {
        Self([
            self[0].min(other[0]), 
            self[1].min(other[1]), 
            self[2].min(other[2]),
        ])
    }
    
    pub fn max(self, other: Self) -> Self {
        Self([
            self[0].max(other[0]), 
            self[1].max(other[1]), 
            self[2].max(other[2]),
        ])
    }

    pub fn abs(self) -> Self {
        Self([
            self[0].abs(), 
            self[1].abs(), 
            self[2].abs(),
        ])
    }

    pub fn mincomp(self) -> Float {
        self[0].min(self[1]).min(self[2])
    }

    pub fn maxcomp(self) -> Float {
        self[0].max(self[1]).max(self[2])
    }
}

impl From<[Float; 3]> for Vec3 {
    fn from(array: [Float; 3]) -> Self {
        Self(array)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
        ])
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self([
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
        ])
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self([
            -self[0], 
            -self[1], 
            -self[2],
        ])
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, other: Float) -> Self::Output {
        Self([
            self[0] * other, 
            self[1] * other, 
            self[2] * other,
        ])
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self([
            self[0] / other[0], 
            self[1] / other[1], 
            self[2] / other[2],
        ])
    }
}

impl Deref for Vec3 {
    type Target = [Float; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
