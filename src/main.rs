mod color;
mod config;
mod image;
mod material;
mod math;
mod random;
mod ray_marcher;
mod ray_tracer;
mod renderer;
mod scene;
mod shapes;

use serde::Deserialize;
use std::{
    fs::File,
    io::BufWriter,
    time::Instant,
};
use config::{Config, ImageConfig};
use image::Image;
use ray_marcher::RayMarcher;
use ray_tracer::RayTracer;
use renderer::{Renderer, PixelRenderer};
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

fn render<P: PixelRenderer>(pixel_renderer: P, scene: &Scene, config: &ImageConfig, path: &str) -> RayResult<()> {
    let mut renderer = Renderer::new(pixel_renderer);
    let start = Instant::now();
    let image = renderer.render(&scene, &config);
    let end = Instant::now();
    let duration = end - start;
    println!("{}.{:09}s total", duration.as_secs(), duration.subsec_nanos());
    save_png(&image, path)
}

fn main() -> RayResult<()> {
    let scene = load_json::<Scene>("scene.json")?;
    let config = load_json::<Config>("config.json")?;
    render(RayTracer::new(config.ray_tracer), &scene, &config.image, "ray-tracer.png")?;
    render(RayMarcher::new(), &scene, &config.image, "ray-marcher.png")?;
    Ok(())
}
