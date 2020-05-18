use crate::{
    math::{Float, Ray, Vector2, Vector3},
    random::Random,
};

pub struct Camera<F> {
    eye: Vector3<F>, 
    right: Vector3<F>,
    up: Vector3<F>,
    forward: Vector3<F>,
    viewport: Vector2<F>,
    lens_radius: F,
    focus: F,
}

impl<F: Float> Camera<F> {
    pub fn new(
        eye: Vector3<F>, 
        target: Vector3<F>, 
        up: Vector3<F>, 
        fov: F, 
        aspect: F,
        aperture: F,
    ) -> Self {
        let theta = fov.to().to_radians();
        let height = F::of(2.0 * (0.5 * theta).tan());
        let viewport = Vector2::new(height * aspect, height);
        let forward = (target - eye).normalized();
        let right = forward.cross(up).normalized();
        let up = right.cross(forward);
        let lens_radius = F::HALF * aperture;
        let focus = (target - eye).length();
        Self {
            eye,
            right,
            up,
            forward,
            viewport,
            lens_radius,
            focus,
        }
    }

    pub fn ray(&self, uv: Vector2<F>, random: &mut Random) -> Ray<F> {
        let rd = random.in_disk() * self.lens_radius;
        let offset = self.right * rd.x + self.up * rd.y;
        Ray {
            origin: self.eye + offset,
            direction: (
                (
                    self.forward + 
                    self.right * self.viewport.x * (uv.x - F::HALF) + 
                    self.up * self.viewport.y * (uv.y - F::HALF)
                ) * self.focus - offset
            ).normalized(),
        }
    } 
}
