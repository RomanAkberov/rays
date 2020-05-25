use smth::Vec3D;
use super::{Bounds3D, Float, Hit, Ray, Sphere};

pub enum Shape {
    Sphere(Sphere),
    Cuboid(Bounds3D),
}

impl Shape {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray),
            Self::Cuboid(aabb) => aabb.hit(ray),
        }
    }

    pub fn sdf(&self, point: Vec3D) -> Float {
        match self {
            Self::Sphere(sphere) => sphere.sdf(point),
            Self::Cuboid(aabb) => aabb.sdf(point),
        }
    }

    pub fn bounds(&self) -> Bounds3D {
        match self {
            Self::Sphere(sphere) => sphere.bounds(),
            Self::Cuboid(aabb) => *aabb,
        }
    }
}
