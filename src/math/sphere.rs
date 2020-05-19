use serde::Deserialize;
use crate::math::{Float, Ray, Vec3};
use super::Hit;

const T_MIN: Float = 0.001;

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
}

impl Sphere {
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
                let point = ray.at(t);
                let normal = (point - self.center).normalized();
                Some(Hit { t, point, normal })
            }
        }
    }

    pub fn sdf(&self, point: Vec3) -> Float {
        self.center.distance(point) - self.radius
    }
}
