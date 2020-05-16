mod color;
mod image;
mod math;
mod scene;

use std::{
    fs::File,
    io::BufWriter,
};
use color::Color;
use image::Image;
use math::Ray;
use scene::Scene;

pub type RayResult<T> = Result<T, Box<dyn std::error::Error>>;

fn trace(ray: Ray, scene: &Scene) -> Color {
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    scene.background.bottom.lerp(&scene.background.top, t)
}

fn render(scene: &Scene, image: &mut Image) {
    for i in 0 .. image.width {
        for j in 0 .. image.height {
            let u = i as f64 / (image.width - 1) as f64;
            let v = j as f64 / (image.height - 1) as f64;
            let ray = scene.camera.ray(u, v);
            let color = trace(ray, scene);
            image.set_color(i, j, color);
        }
    }
}

fn save_as_png(image: &Image, path: &str) -> RayResult<()> {
    let file = File::create(path)?;
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, image.width, image.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut bytes = vec![0u8; 3 * image.colors.len()];
    for (i, color) in image.colors.iter().enumerate() {
        bytes[3 * i] = (color.r * 255.0) as u8;
        bytes[3 * i + 1] = (color.g * 255.0) as u8;
        bytes[3 * i + 2] = (color.b * 255.0) as u8;
    }
    writer.write_image_data(&bytes)?;
    Ok(())
}

fn load_from_json(path: &str) -> RayResult<Scene> {
    let file = File::open(path)?;
    let scene = serde_json::from_reader(file)?;
    Ok(scene)
}

fn main() -> RayResult<()> {
    let scene = load_from_json("scene.json")?;
    let mut image = Image::new(800, 450);
    render(&scene, &mut image);
    save_as_png(&image, "output.png")
}
