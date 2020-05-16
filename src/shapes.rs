use serde::Deserialize;
use crate::math::{Vector3, Ray};

pub struct Hit {
}

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            Some(Hit {})
        } else {
            None
        }
    }
}
