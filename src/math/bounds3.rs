use smth::{Vec3, Vec3D};
use super::{Float, Hit, Ray};

#[derive(Copy, Clone)]
pub struct Bounds3<A> {
    pub min: Vec3<A>,
    pub max: Vec3<A>,
}

pub type Bounds3D = Bounds3<f64>;

impl Bounds3<f64> {
    pub fn center(&self) -> Vec3D {
        (self.min + self.max) * 0.5
    }

    pub fn diagonal(&self) -> Vec3D {
        self.max - self.min
    }

    pub fn sdf(&self, point: Vec3D) -> Float {
        let p = point - self.center();
        let q = p.abs() - self.diagonal() * 0.5;
        q.max(Vec3::ZERO).length() + q.max_component().min(0.0)
    }

    pub fn hit(&self, _: &Ray) -> Option<Hit> {
        None
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let inverse = Vec3D::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        ); // TODO precompute
        let t1 = (self.min - ray.origin) * inverse;
        let t2 = (self.max - ray.origin) * inverse;
        let tmin = t1.min(t2).max_component();
        let tmax = t1.max(t2).min_component();
        tmax > tmin.max(0.0)
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}
