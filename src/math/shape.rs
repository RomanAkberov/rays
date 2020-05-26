use smth::{Float, Vec3};
use super::{Bounds3, Hit, Ray, Sphere};

pub enum Shape<S> {
    Sphere(Sphere<S>),
    Cuboid(Bounds3<S>),
}

impl<S: Float> Shape<S> {
    pub fn hit(&self, ray: &Ray<S>) -> Option<Hit<S>> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray),
            Self::Cuboid(aabb) => aabb.hit(ray),
        }
    }

    pub fn sdf(&self, point: Vec3<S>) -> S {
        match self {
            Self::Sphere(sphere) => sphere.sdf(point),
            Self::Cuboid(aabb) => aabb.sdf(point),
        }
    }

    pub fn bounds(&self) -> Bounds3<S> {
        match self {
            Self::Sphere(sphere) => sphere.bounds(),
            Self::Cuboid(aabb) => *aabb,
        }
    }
}
