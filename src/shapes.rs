use serde::Deserialize;
use crate::math::{Vector3, Ray};

const T_MIN: f64 = 0.001;

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
    pub fn distance(&self, point: Vector3) -> f64 {
        self.center.distance(point) - self.radius
    }

    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - a * c;
        if d < 0.0 {
            None
        } else {
            let d_sqrt = d.sqrt(); 
            let mut t = (-b - d_sqrt) / a;
            if t <= T_MIN {
                t = (-b + d_sqrt) / a;
            }
            if t <= T_MIN {
                None
            } else {
                let normal = (ray.at(t) - self.center).normalized();
                Some(Hit { t, normal })
            }
        }
    }
}
