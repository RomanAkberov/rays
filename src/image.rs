use crate::color::Color;

pub struct Image<S> {
    pub width: u32,
    pub height: u32,
    pub colors: Vec<Color<S>>,
}
