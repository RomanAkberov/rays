use serde::Deserialize;
use crate::math::{Vector3, Ray};

pub struct Hit {
    pub t: f64,
    pub normal: Vector3,
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
        let b = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / a;
            let normal = (ray.at(t) - self.center).normalized();
            Some(Hit { t, normal })
        }
    }
}
