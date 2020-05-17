use crate::math::{Float, Ray, Vector2, Vector3};

pub struct Camera<F> {
    eye: Vector3<F>, 
    right: Vector3<F>,
    up: Vector3<F>,
    forward: Vector3<F>,
    viewport: Vector2<F>,
}

impl<F: Float> Camera<F> {
    pub fn new(eye: Vector3<F>, target: Vector3<F>, up: Vector3<F>, fov: F, aspect: F) -> Self {
        let theta = fov.to().to_radians();
        let height = F::of(2.0 * (0.5 * theta).tan());
        let viewport = Vector2::new(height * aspect, height);
        let forward = (target - eye).normalized();
        let right = forward.cross(up).normalized();
        let up = right.cross(forward);
        Self {
            eye,
            right,
            up,
            forward,
            viewport,
        }
    }

    pub fn ray(&self, uv: Vector2<F>) -> Ray<F> {
        Ray {
            origin: self.eye,
            direction: (
                self.forward + 
                self.right * self.viewport.x * (uv.x - F::HALF) + 
                self.up * self.viewport.y * (uv.y - F::HALF)
            ).normalized(),
        }
    } 
}
