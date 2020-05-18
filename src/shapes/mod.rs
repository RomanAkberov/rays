mod sphere;

use crate::math::{Aabb3, Float, Ray, Vec3};

pub use sphere::Sphere;

pub struct Hit {
    pub t: Float,
    pub normal: Vec3,
}

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn sdf(&self, point: Vec3) -> Float;
}

impl Shape for Aabb3 {
    fn hit(&self, _ray: &Ray) -> Option<Hit> {
        None
    }

    fn sdf(&self, point: Vec3) -> Float {
        let p = point - self.center();
        let q = p.abs() - self.diagonal() * 0.5;
        q.max(Vec3::ZERO).len() + q.x.max(q.y).max(q.z).min(0.0)
    }
}
