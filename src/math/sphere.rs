use serde::Deserialize;
use smth::Vec3D;
use crate::math::{Bounds3D, Float, Ray};
use super::Hit;

const T_MIN: Float = 0.001;

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Vec3D,
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
            return None;
        }
        let d_sqrt = d.sqrt(); 
        let mut t = (-b - d_sqrt) / a;
        if t <= T_MIN {
            t = (-b + d_sqrt) / a;
        }
        if t <= T_MIN {
            return None;
        } 
        let point = ray.at(t);
        let normal = (point - self.center).normalized();
        Some(Hit { t, point, normal })
    }

    pub fn sdf(&self, point: Vec3D) -> Float {
        self.center.distance(point) - self.radius
    }

    pub fn bounds(&self) -> Bounds3D {
        let half_extent = Vec3D::of(self.radius);
        Bounds3D {
            min: self.center - half_extent,
            max: self.center + half_extent, 
        }
    }
}
