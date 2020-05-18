use serde::Deserialize;
use crate::{
    color::Color,
    math::Ray,
};

#[derive(Deserialize)]
pub struct Background {
    pub top: Color,
    pub bottom: Color,
}

impl Background {
    pub fn color(&self, ray: &Ray) -> Color {
        let t = 0.5 * (ray.direction[1] + 1.0);
        self.bottom.lerp(&self.top, t)
    }
}
