use super::Vec3;

pub struct Aabb3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb3 {
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }
}
