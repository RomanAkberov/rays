use serde::Deserialize;
use smth::Float;
use crate::{
    color::Color,
    math::Ray,
};

#[derive(Deserialize)]
pub struct Background<S> {
    pub top: Color<S>,
    pub bottom: Color<S>,
}

impl<S: Float> Background<S> {
    pub fn color(&self, ray: &Ray<S>) -> Color<S> {
        let t = S::HALF * (ray.direction.y + S::ONE);
        self.bottom.lerp(&self.top, t)
    }
}
