use crate::math::{Float, Ray, Vector2, Vector3};

pub struct Camera<F> {
    eye: Vector3<F>, 
    right: Vector3<F>,
    up: Vector3<F>,
    focal_length: F,
}

impl<F: Float> Camera<F> {
    pub fn new(fov: F, aspect: F) -> Self {
        let theta = fov.to().to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = F::of(2.0 * h);
        let viewport_width = aspect * viewport_height;
        let focal_length = F::ONE;
        let eye = Vector3::ZERO;
        let right = Vector3::new(viewport_width, F::ZERO, F::ZERO);
        let up = Vector3::new(F::ZERO, viewport_height, F::ZERO);
        Self {
            eye,
            right,
            up,
            focal_length,
        }
    }

    pub fn ray(&self, uv: Vector2<F>) -> Ray<F> {
        Ray {
            origin: self.eye,
            direction: (self.right * (uv.x - F::HALF) + self.up * (uv.y - F::HALF) - Vector3::new(F::ZERO, F::ZERO, self.focal_length)).normalized(),
        }
    } 
}
