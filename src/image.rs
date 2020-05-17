use crate::color::Color;
use crate::math::Float;

pub struct Image<F: Float> {
    pub width: u32,
    pub height: u32,
    pub colors: Vec<Color<F>>,
}
