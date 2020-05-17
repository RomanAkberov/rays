use serde::Deserialize;
use crate::{
    color::Color,
    material::Material,
    math::{Float, Ray, Vector2, Vector3},
    shapes::Sphere,
};

#[derive(Deserialize)]
pub struct Scene<F> {
    pub camera: Camera<F>,
    pub background: Background<F>,
    pub objects: Vec<SceneObject<F>>,
}

#[derive(Deserialize)]
pub struct SceneObject<F> {
    pub shape: Sphere<F>,
    pub material: Material<F>,
}

#[derive(Deserialize)]
pub struct Background<F> {
    pub top: Color<F>,
    pub bottom: Color<F>,
}

impl<F: Float> Background<F> {
    pub fn color(&self, ray: &Ray<F>) -> Color<F> {
        let t = F::HALF * (ray.direction.y + F::ONE);
        self.bottom.lerp(&self.top, t)
    }
}

#[derive(Deserialize)]
pub struct Camera<F> {
    pub eye: Vector3<F>, 
    pub right: Vector3<F>,
    pub up: Vector3<F>,
}

impl<F: Float> Camera<F> {
    pub fn ray(&self, uv: Vector2<F>) -> Ray<F> {
        Ray {
            origin: self.eye,
            direction: (self.right * (uv.x - F::HALF) + self.up * (uv.y - F::HALF) - Vector3::new(F::ZERO, F::ZERO, F::ONE)).normalized(),
        }
    } 
}
