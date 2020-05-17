mod color;
mod config;
mod image;
mod material;
mod math;
mod path_tracer;
mod random;
mod renderer;
mod scene;
mod shapes;

use serde::Deserialize;
use std::{
    fs::File,
    io::BufWriter,
    time::Instant,
};
use config::Config;
use image::Image;
use path_tracer::PathTracer;
use renderer::Renderer;
use scene::Scene;

pub type RayResult<T> = Result<T, Box<dyn std::error::Error>>;

fn save_png(image: &Image, path: &str) -> RayResult<()> {
    let file = File::create(path)?;
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, image.width, image.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut bytes = vec![0u8; 3 * image.colors.len()];
    for (i, color) in image.colors.iter().enumerate() {
        bytes[3 * i] = (color.r.min(1.0) * 255.0) as u8;
        bytes[3 * i + 1] = (color.g.min(1.0) * 255.0) as u8;
        bytes[3 * i + 2] = (color.b.min(1.0) * 255.0) as u8;
    }
    writer.write_image_data(&bytes)?;
    Ok(())
}

fn load_json<T: for <'d> Deserialize<'d>>(path: &str) -> RayResult<T> {
    let file = File::open(path)?;
    let value = serde_json::from_reader(file)?;
    Ok(value)
}

fn main() -> RayResult<()> {
    let scene = load_json::<Scene>("scene.json")?;
    let config = load_json::<Config>("config.json")?;
    let mut renderer = PathTracer::new(config);
    let start = Instant::now();
    let image = renderer.render(&scene);
    let end = Instant::now();
    let duration = end - start;
    println!("{}.{:09}s total", duration.as_secs(), duration.subsec_nanos());
    save_png(&image, "output.png")
}
