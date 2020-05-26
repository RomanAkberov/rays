use smth::{Float, Vec2, Vec3};
use crate::{
    math::Ray,
    random::Random,
};

pub struct Camera<S> {
    eye: Vec3<S>, 
    right: Vec3<S>,
    up: Vec3<S>,
    forward: Vec3<S>,
    viewport: Vec2<S>,
    lens_radius: S,
    focus: S,
}

impl<S: Float> Camera<S> {
    pub fn new(
        eye: Vec3<S>, 
        target: Vec3<S>, 
        up: Vec3<S>, 
        fov: S, 
        aspect: S,
        aperture: S,
    ) -> Self {
        let theta = fov.to_radians();
        let height = S::TWO * (S::HALF * theta).tan();
        let viewport = Vec2::new(height * aspect, height);
        let forward = (target - eye).normalized();
        let right = forward.cross(up).normalized();
        let up = right.cross(forward);
        let lens_radius = S::HALF * aperture;
        let focus = target.distance(eye);
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

    pub fn ray(&self, uv: Vec2<S>, random: &mut Random) -> Ray<S> {
        let rd = random.in_disk() * self.lens_radius;
        let offset = self.right * rd.x + self.up * rd.y;
        Ray {
            origin: self.eye + offset,
            direction: (
                (
                    self.forward + 
                    self.right * self.viewport.x * (uv.x - S::HALF) + 
                    self.up * self.viewport.y * (uv.y - S::HALF)
                ) * self.focus - offset
            ).normalized(),
        }
    } 
}
