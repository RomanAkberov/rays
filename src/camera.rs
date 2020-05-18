use crate::{
    math::{Float, Ray, Vec2, Vec3},
    random::Random,
};

pub struct Camera {
    eye: Vec3, 
    right: Vec3,
    up: Vec3,
    forward: Vec3,
    viewport: Vec2,
    lens_radius: Float,
    focus: Float,
}

impl Camera {
    pub fn new(
        eye: Vec3, 
        target: Vec3, 
        up: Vec3, 
        fov: Float, 
        aspect: Float,
        aperture: Float,
    ) -> Self {
        let theta = fov.to_radians();
        let height = 2.0 * (0.5 * theta).tan();
        let viewport = Vec2([height * aspect, height]);
        let forward = (target - eye).normalized();
        let right = forward.cross(up).normalized();
        let up = right.cross(forward);
        let lens_radius = 0.5 * aperture;
        let focus = (target - eye).len();
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

    pub fn ray(&self, uv: Vec2, random: &mut Random) -> Ray {
        let rd = random.in_disk() * self.lens_radius;
        let offset = self.right * rd[0] + self.up * rd[1];
        Ray {
            origin: self.eye + offset,
            direction: (
                (
                    self.forward + 
                    self.right * self.viewport[0] * (uv[0] - 0.5) + 
                    self.up * self.viewport[1] * (uv[1] - 0.5)
                ) * self.focus - offset
            ).normalized(),
        }
    } 
}
