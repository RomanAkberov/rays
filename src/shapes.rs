use serde::Deserialize;
use crate::math::{Float, Vector3, Ray};

const T_MIN: f64 = 0.001;

pub struct Hit<F> {
    pub t: F,
    pub normal: Vector3<F>,
}

#[derive(Deserialize)]
pub struct Sphere<F> {
    pub center: Vector3<F>,
    pub radius: F,
}

impl<F: Float> Sphere<F> {
    pub fn distance(&self, point: Vector3<F>) -> F {
        self.center.distance(point) - self.radius
    }

    pub fn hit(&self, ray: &Ray<F>) -> Option<Hit<F>> {
        let t_min = F::of(T_MIN);
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - a * c;
        if d < F::ZERO {
            None
        } else {
            let d_sqrt = d.sqrt(); 
            let mut t = (-b - d_sqrt) / a;
            if t <= t_min {
                t = (-b + d_sqrt) / a;
            }
            if t <= t_min {
                None
            } else {
                let normal = (ray.at(t) - self.center).normalized();
                Some(Hit { t, normal })
            }
        }
    }
}
