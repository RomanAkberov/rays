use smth::Vec3D;
use super::Float;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn at(&self, t: Float) -> Vec3D {
        self.origin + self.direction * t
    }
}
