use std::ops::{Add, Sub, Mul, Neg};
use serde::Deserialize;
use super::Float;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Deserialize)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub fn normalized(self) -> Self {
        let scale = 1.0 / self.len();
        Self::new(self.x * scale, self.y * scale, self.z * scale)
    }

    pub fn len(self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn len2(self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn distance(self, other: Self) -> Float {
        (self - other).len()
    }

    pub fn dot(self, other: Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl From<[Float; 3]> for Vec3 {
    fn from(array: [Float; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, other: Float) -> Self::Output {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
