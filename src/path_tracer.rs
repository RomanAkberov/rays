use std::time::Instant;
use crate::{
    color::Color,
    config::Config,
    image::Image,
    math::Ray,
    random::Random,
    renderer::Renderer,
    scene::Scene,
};

pub struct PathTracer {
    config: Config,
    random: Random,
}

impl PathTracer {
    pub fn new(config: Config) -> Self {
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
        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        scene.background.bottom.lerp(&scene.background.top, t)
    }
}

impl Renderer for PathTracer {
    fn render(&mut self, scene: &Scene) -> Image {
        let width = self.config.width;
        let height = self.config.height;
        let scale = 1.0 / self.config.samples as f64;
        let mut colors = Vec::with_capacity((width * height) as usize);
        for j in 0 .. height {
            for i in 0 .. width {
                let mut color = Color::BLACK;
                for _ in 0 .. self.config.samples {
                    let u = (i as f64 + self.random.range01()) / (width - 1) as f64;
                    let v = 1.0 - (j as f64 + self.random.range01()) / (height - 1) as f64;
                    let ray = scene.camera.ray(u, v);
                    color = color + self.cast_ray(&ray, scene, self.config.max_depth);
                }
                color = color * scale;
                if self.config.gamma_correction {
                    color.r = color.r.sqrt();
                    color.g = color.g.sqrt();
                    color.b = color.b.sqrt();
                }
                colors.push(color);
            }
        }
        Image { width, height, colors }
    }
}
