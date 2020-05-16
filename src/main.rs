mod color;
mod config;
mod image;
mod math;
mod random;
mod scene;
mod shapes;

use serde::Deserialize;
use std::{
    fs::File,
    io::BufWriter,
    time::Instant,
};
use color::Color;
use config::Config;
use image::Image;
use math::Ray;
use random::Random;
use scene::Scene;

pub type RayResult<T> = Result<T, Box<dyn std::error::Error>>;

fn trace(ray: &Ray, scene: &Scene) -> Color {
    let hit = scene.shapes
        .iter()
        .flat_map(|shape| shape.hit(&ray))
        .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap());
    if let Some(hit) = hit {
        return Color::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5;
    }
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    scene.background.bottom.lerp(&scene.background.top, t)
}

fn render(scene: &Scene, config: &Config) -> Image {
    let start = Instant::now();
    let mut image = Image::new(config.width, config.height);
    let mut random = Random::new(42);
    let scale = 1.0 / config.samples as f64;
    for i in 0 .. image.width {
        for j in 0 .. image.height {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0 .. config.samples {
                let u = (i as f64 + random.next_f64()) / (image.width - 1) as f64;
                let v = 1.0 - (j as f64 + random.next_f64()) / (image.height - 1) as f64;
                let ray = scene.camera.ray(u, v);
                color = color + trace(&ray, scene);
            }
            image.set_color(i, j, color * scale);
        }
    }
    let end = Instant::now();
    let total = end - start;
    let num_rays = config.width * config.height * config.samples;
    let per_ray = total / num_rays;
    println!("{}.{:09}s total", total.as_secs(), total.subsec_nanos());
    println!("{}.{:09}s per ray", per_ray.as_secs(), per_ray.subsec_nanos());
    image
}

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
    let image = render(&scene, &config);
    save_png(&image, "output.png")
}
