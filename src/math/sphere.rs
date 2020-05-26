use serde::Deserialize;
use smth::{Float, Vec3};
use crate::math::{Bounds3, Ray};
use super::Hit;


#[derive(Deserialize)]
pub struct Sphere<S> {
    pub center: Vec3<S>,
    pub radius: S,
}

impl<S: Float> Sphere<S> {
    pub fn hit(&self, ray: &Ray<S>) -> Option<Hit<S>> {
        let t_min = S::of(0.001);
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - a * c;
        if d < S::ZERO {
            return None;
        }
        let d_sqrt = d.sqrt(); 
        let mut t = (-b - d_sqrt) / a;
        if t <= t_min {
            t = (-b + d_sqrt) / a;
        }
        if t <= t_min {
            return None;
        } 
        let point = ray.at(t);
        let normal = (point - self.center).normalized();
        Some(Hit { t, point, normal })
    }

    pub fn sdf(&self, point: Vec3<S>) -> S {
        self.center.distance(point) - self.radius
    }

    pub fn bounds(&self) -> Bounds3<S> {
        let half_extent = Vec3::of(self.radius);
        Bounds3 {
            min: self.center - half_extent,
            max: self.center + half_extent, 
        }
    }
}
