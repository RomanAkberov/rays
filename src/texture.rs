use smth::Float;
use crate::{
    color::Color,
    math::Hit,
};

pub enum Texture<S> {
    Constant(Color<S>),
    Normal,
    Checker(Box<Texture<S>>, Box<Texture<S>>, S),
}

impl<S: Float> Texture<S> {
    pub fn color(&self, hit: &Hit<S>) -> Color<S> {
        match self {
            &Self::Constant(color) => color,
            Self::Normal => {
                Color::new(
                    hit.normal.x * S::HALF + S::ONE,
                    hit.normal.y * S::HALF + S::ONE,
                    hit.normal.z * S::HALF + S::ONE,
                )
            }
            Self::Checker(even, odd, scale) => {
                let p = hit.point * *scale;
                let sines = p.x.sin() * p.y.sin() * p.z.sin();
                if sines < S::ZERO {
                    even.color(hit)
                } else {
                    odd.color(hit)
                }
            }
        }
    }
}
