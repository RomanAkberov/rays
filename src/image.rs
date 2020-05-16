use std::{
    fs::File,
    io::BufWriter,
};
use crate::{
    RayResult,
    color::Color,
};

pub struct Image {
    pub colors: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn save_as_png(&self, path: &str) -> RayResult {
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        let mut bytes = vec![0u8; 3 * self.colors.len()];
        for (i, color) in self.colors.iter().enumerate() {
            bytes[3 * i] = (color.r * 255.0) as u8;
            bytes[3 * i + 1] = (color.g * 255.0) as u8;
            bytes[3 * i + 2] = (color.b * 255.0) as u8;
        }
        writer.write_image_data(&bytes)?;
        Ok(())
    }
}
