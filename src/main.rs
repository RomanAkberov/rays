mod background;
mod camera;
mod color;
mod config;
mod def;
mod image;
mod material;
mod math;
mod object;
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
use config::Config;
use def::SceneDef;
use image::Image;
use math::Float;
use ray_marcher::RayMarcher;
use ray_tracer::RayTracer;
use renderer::{Renderer, PixelRenderer, Supersampler};
use scene::Scene;

pub type RayResult<T> = Result<T, Box<dyn std::error::Error>>;

fn save_png<F: Float>(image: &Image<F>, path: &str) -> RayResult<()> {
    let file = File::create(path)?;
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, image.width, image.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut bytes = vec![0u8; 3 * image.colors.len()];
    for (i, color) in image.colors.iter().enumerate() {
        bytes[3 * i] = (color.r.to().min(1.0) * 255.0) as u8;
        bytes[3 * i + 1] = (color.g.to().min(1.0) * 255.0) as u8;
        bytes[3 * i + 2] = (color.b.to().min(1.0) * 255.0) as u8;
    }
    writer.write_image_data(&bytes)?;
    Ok(())
}

fn load_json<T: for <'d> Deserialize<'d>>(path: &str) -> RayResult<T> {
    let file = File::open(path)?;
    let value = serde_json::from_reader(file)?;
    Ok(value)
}

fn render<F: Float, P: PixelRenderer>(pixel_renderer: P, scene: &Scene<F>, config: &Config, path: &str) -> RayResult<()> {
    let mut renderer = Renderer::new(Supersampler::new(pixel_renderer, config.samples));
    let start = Instant::now();
    let image = renderer.render(&scene, &config.image);
    let end = Instant::now();
    let duration = end - start;
    println!("{}.{:09}s total", duration.as_secs(), duration.subsec_nanos());
    save_png(&image, path)
}

fn run<F: Float>(config: &Config) -> RayResult<()> {
    let def = load_json::<SceneDef<F>>("scene.json")?;
    let aspect = F::of(config.image.width as f64 / config.image.height as f64);
    let scene = Scene::load(def, aspect);
    render(RayTracer::new(config.max_depth), &scene, &config, "ray-tracer.png")?;
    render(RayMarcher::new(), &scene, &config, "ray-marcher.png")?;
    Ok(())
}

fn main() -> RayResult<()> {
    let config = load_json::<Config>("config.json")?;
    match config.precision {
        config::Precision::F32 => run::<f32>(&config),
        config::Precision::F64 => run::<f64>(&config),
    }
}
