use super::{Aabb3, Float, Hit, Ray, Sphere, Vec3};

pub enum Shape {
    Sphere(Sphere),
    Cuboid(Aabb3),
}

impl Shape {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray),
            Self::Cuboid(aabb) => aabb.hit(ray),
        }
    }

    pub fn sdf(&self, point: Vec3) -> Float {
        match self {
            Self::Sphere(sphere) => sphere.sdf(point),
            Self::Cuboid(aabb) => aabb.sdf(point),
        }
    }
}
