use crate::{
    color::Color,
    shapes::Hit,
};

pub trait Texture: Sync {
    fn value(&self, hit: &Hit) -> Color;
}

impl Texture for Color {
    fn value(&self, _: &Hit) -> Color {
        *self
    }
}

pub struct Normal;

impl Texture for Normal {
    fn value(&self, hit: &Hit) -> Color {
        Color::new(
            hit.normal[0] * 0.5 + 1.0,
            hit.normal[1] * 0.5 + 1.0,
            hit.normal[2] * 0.5 + 1.0,
        )
    }
}

pub struct Checker<T0, T1>(pub T0, pub T1);

impl<T0: Texture, T1: Texture> Texture for Checker<T0, T1> {
    fn value(&self, hit: &Hit) -> Color {
        let p = hit.point * 10.0;
        let sines = p[0].sin() * p[1].sin() * p[2].sin();
        if sines < 0.0 {
            self.0.value(hit)
        } else {
            self.1.value(hit)
        }
    }
}
