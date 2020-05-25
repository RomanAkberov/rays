use crate::{
    color::Color,
    math::Hit,
};

pub enum Texture {
    Constant(Color),
    Normal,
    Checker(Box<Texture>, Box<Texture>),
}

impl Texture {
    pub fn color(&self, hit: &Hit) -> Color {
        match self {
            &Self::Constant(color) => color,
            Self::Normal => {
                Color::new(
                    hit.normal.x * 0.5 + 1.0,
                    hit.normal.y * 0.5 + 1.0,
                    hit.normal.z * 0.5 + 1.0,
                )
            }
            Self::Checker(even, odd) => {
                let p = hit.point * 10.0;
                let sines = p.x.sin() * p.y.sin() * p.z.sin();
                if sines < 0.0 {
                    even.color(hit)
                } else {
                    odd.color(hit)
                }
            }
        }
    }
}
