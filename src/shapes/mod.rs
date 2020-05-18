mod sphere;

use crate::math::{Aabb3, Float, Ray, Vec3};

pub use sphere::Sphere;

pub struct Hit {
    pub t: Float,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn sdf(&self, point: Vec3) -> Float;
}

impl Shape for Aabb3 {
    fn hit(&self, _: &Ray) -> Option<Hit> {
        // let tmin = (self.min - ray.origin) / ray.direction;
        // let tmax = (self.max - ray.origin) / ray.direction;
        // let rmin = tmin.min(tmax); 
        // let rmax = tmin.max(tmax);
        // let minmax = rmin.maxcomp();
        // let maxmin = rmax.mincomp();
        // if minmax >= maxmin {
        //     Some(Hit { t: maxmin, normal: Vec3::new(0.0, 1.0, 0.0) })
        // } else {
        //     None
        // }
        None
    }

    fn sdf(&self, point: Vec3) -> Float {
        let p = point - self.center();
        let q = p.abs() - self.diagonal() * 0.5;
        q.max(Vec3::ZERO).len() + q.maxcomp().min(0.0)
    }
}
