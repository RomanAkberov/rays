mod color;
mod image;

use color::Color;
use image::Image;

pub type RayResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> RayResult {
    let image = Image {
        colors: vec![
            Color::new(1.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0),
            Color::new(0.0, 1.0, 0.0), Color::new(0.0, 0.0, 1.0)
        ],
        width: 2,
        height: 2,
    };
    image.save_as_png("output.png")
}
