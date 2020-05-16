use crate::color::Color;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub colors: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            colors: vec![Color::default(); (width * height) as usize],
        }
    }

    pub fn set_color(&mut self, i: u32, j: u32, color: Color) {
        self.colors[(i + j * self.width) as usize] = color;
    }
}
