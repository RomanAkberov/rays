use super::{Float, Hit, Ray, Vec3};

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

    pub fn sdf(&self, point: Vec3) -> Float {
        let p = point - self.center();
        let q = p.abs() - self.diagonal() * 0.5;
        q.max(Vec3::ZERO).len() + q.maxcomp().min(0.0)
    }

    pub fn hit(&self, _: &Ray) -> Option<Hit> {
        None
    }
}
