mod background;
mod camera;
mod color;
mod config;
mod def;
mod image;
mod material;
mod math;
mod object;
mod progress;
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
use image::Image;
use ray_marcher::RayMarcher;
use ray_tracer::RayTracer;
use progress::{Progress, ConsoleProgress, NoProgress};
use renderer::{Renderer, PixelRenderer, Supersampler};
use scene::Scene;

pub use background::Background;
pub use color::Color;
pub use config::{Config, ImageConfig, Precision, RenderMode};
pub use def::{SceneDef, CameraDef};
pub use material::{Material, Mode};
pub use math::{Float, Vector3};
pub use object::Object;
pub use random::Random;
pub use shapes::Sphere;

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

pub fn load_json<T: for <'d> Deserialize<'d>>(path: &str) -> RayResult<T> {
    let file = File::open(path)?;
    let value = serde_json::from_reader(file)?;
    Ok(value)
}

fn render<F: Float, P: PixelRenderer>(pixel_renderer: P, scene: &Scene<F>, config: &Config) -> RayResult<()> {
    let mut renderer = Renderer::new(Supersampler::new(pixel_renderer, config.samples));
    let start = Instant::now();
    let progress = if config.show_progress {
        Box::new(ConsoleProgress::default()) as Box<dyn Progress>
    } else {
        Box::new(NoProgress::default()) as Box<dyn Progress>
    };
    let image = renderer.render(&scene, &config.image, &*progress);
    let end = Instant::now();
    let duration = end - start;
    println!("{}.{:09}s total", duration.as_secs(), duration.subsec_nanos());
    save_png(&image, &config.image.path)
}

pub fn run_scene<F: Float>(config: &Config, scene: SceneDef<F>) -> RayResult<()> {
    let aspect = F::of(config.image.width as f64 / config.image.height as f64);
    let scene = Scene::load(scene, aspect);
    match config.renderer {
        RenderMode::RayTracer => render(RayTracer::new(config.max_depth), &scene, &config)?,
        RenderMode::RayMarcher => render(RayMarcher::new(), &scene, &config)?,
    }
    Ok(())
}

pub fn run_loader<L: SceneLoader>(config: &Config, loader: L) -> RayResult<()> {
    match config.precision {
        config::Precision::F32 => run_scene(config, loader.load::<f32>()?),
        config::Precision::F64 => run_scene(config, loader.load::<f64>()?),
    }
}

pub fn run_file(config: &Config, path: &str) -> RayResult<()> {
    run_loader(config, FileLoader { path })
}

pub trait SceneLoader {
    fn load<F: Float>(self) -> RayResult<SceneDef<F>>; 
}

pub struct FileLoader<'a> {
    path: &'a str,
}

impl<'a> SceneLoader for FileLoader<'a> {
    fn load<F: Float>(self) -> RayResult<SceneDef<F>> {
        load_json(self.path)
    }
}
