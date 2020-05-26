use smth::{Float, Vec3};
use super::{Hit, Ray};

#[derive(Copy, Clone)]
pub struct Bounds3<S> {
    pub min: Vec3<S>,
    pub max: Vec3<S>,
}

impl<S: Float> Bounds3<S> {
    pub fn center(&self) -> Vec3<S> {
        (self.min + self.max) * S::HALF
    }

    pub fn diagonal(&self) -> Vec3<S> {
        self.max - self.min
    }

    pub fn sdf(&self, point: Vec3<S>) -> S {
        let p = point - self.center();
        let q = p.abs() - self.diagonal() * S::HALF;
        q.max(Vec3::ZERO).len() + q.max_component().min(S::ZERO)
    }

    pub fn hit(&self, _: &Ray<S>) -> Option<Hit<S>> {
        None
    }

    pub fn intersects(&self, ray: &Ray<S>) -> bool {
        let inverse = Vec3 {
            x: S::ONE / ray.direction.x,
            y: S::ONE / ray.direction.y,
            z: S::ONE / ray.direction.z,
        }; // TODO precompute
        let t1 = (self.min - ray.origin) * inverse;
        let t2 = (self.max - ray.origin) * inverse;
        let tmin = t1.min(t2).max_component();
        let tmax = t1.max(t2).min_component();
        tmax > tmin.max(S::ZERO)
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}
