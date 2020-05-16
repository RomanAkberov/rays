use serde::Deserialize;
use crate::{
    color::Color,
    math::{Vector3, Ray},
    shapes::Sphere,
};

#[derive(Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub background: Background,
    pub shapes: Vec<Sphere>,
}

#[derive(Deserialize)]
pub struct Background {
    pub top: Color,
    pub bottom: Color,
}

#[derive(Deserialize)]
pub struct Camera {
    pub eye: Vector3, 
    pub right: Vector3,
    pub up: Vector3,
}

impl Camera {
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.eye,
            direction: self.right * (u - 0.5) + self.up * (v - 0.5) - Vector3::new(0.0, 0.0, 1.0)
        }
    } 
}