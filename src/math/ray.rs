use super::{Float, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + self.direction * t
    }
}
