use serde::Deserialize;
use crate::{
    color::Color,
    math::{Float, Ray},
};

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
