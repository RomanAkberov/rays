use serde::Deserialize;
use crate::{
    color::Color,
    math::Ray,
    random::Random,
    renderer::PixelRenderer,
    scene::Scene,
};

#[derive(Deserialize)]
pub struct RayTracerConfig {
    pub samples: u32,
    pub max_depth: u32,
}


pub struct RayTracer {
    config: RayTracerConfig,
    random: Random,
}

impl RayTracer {
    pub fn new(config: RayTracerConfig) -> Self {
        Self {
            config,
            random: Random::from_seed(42),
        }
    }

    fn cast_ray(&mut self, ray: &Ray, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }
        let hit = scene.objects
            .iter()
            .flat_map(|object| object.shape.hit(&ray).map(|hit| (hit, &object.material)))
            .min_by(|hit1, hit2| hit1.0.t.partial_cmp(&hit2.0.t).unwrap());
        if let Some((hit, material)) = hit {
            if let Some((color, ray)) = material.scatter(ray, &hit, &mut self.random) {
                return color * self.cast_ray(&ray, scene, depth - 1);
            } else {
                return Color::BLACK;
            }
        }
        let unit_direction = ray.direction;
        let t = 0.5 * (unit_direction.y + 1.0);
        scene.background.bottom.lerp(&scene.background.top, t)
    }
}

impl PixelRenderer for RayTracer {
    fn render_pixel(&mut self, scene: &Scene, pixel: (u32, u32), size: (u32, u32)) -> Color {
        let scale = 1.0 / self.config.samples as f64;
        let mut color = Color::BLACK;
        for _ in 0 .. self.config.samples {
            let u = (pixel.0 as f64 + self.random.range01()) / (size.0 - 1) as f64;
            let v = 1.0 - (pixel.1 as f64 + self.random.range01()) / (size.1 - 1) as f64;
            let ray = scene.camera.ray(u, v);
            color = color + self.cast_ray(&ray, scene, self.config.max_depth);
        }
        color * scale
    }
}
