use serde::Deserialize;
use crate::{
    color::Color,
    material::Material,
    math::{Ray, Vector2, Vector3},
    shapes::Sphere,
};

#[derive(Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub background: Background,
    pub objects: Vec<SceneObject>,
}

#[derive(Deserialize)]
pub struct SceneObject {
    pub shape: Sphere,
    pub material: Material,
}

#[derive(Deserialize)]
pub struct Background {
    pub top: Color,
    pub bottom: Color,
}

impl Background {
    pub fn color(&self, ray: &Ray) -> Color {
        let t = 0.5 * (ray.direction.y + 1.0);
        self.bottom.lerp(&self.top, t)
    }
}

#[derive(Deserialize)]
pub struct Camera {
    pub eye: Vector3, 
    pub right: Vector3,
    pub up: Vector3,
}

impl Camera {
    pub fn ray(&self, uv: Vector2) -> Ray {
        Ray {
            origin: self.eye,
            direction: (self.right * (uv.x - 0.5) + self.up * (uv.y - 0.5) - Vector3::new(0.0, 0.0, 1.0)).normalized(),
        }
    } 
}