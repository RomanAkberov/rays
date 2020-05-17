use crate::{
    color::Color,
    random::Random,
    renderer::{PixelRenderer, Pixel},
    scene::Scene,
};

pub const MIN_T: f64 = 0.0001;
pub const MAX_T: f64 = 100.0;

pub struct RayMarcher {}

impl RayMarcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl PixelRenderer for RayMarcher {
    fn render_pixel(&self, scene: &Scene, pixel: Pixel, random: &mut Random) -> Color {
        let uv = random.in_pixel(pixel);
        let mut ray = scene.camera.ray(uv);
        loop {
            let hit = scene.objects
                .iter()
                .map(|obj| (obj, obj.shape.distance(ray.origin)))
                .min_by(|hit1, hit2| hit1.1.partial_cmp(&hit2.1).unwrap());
            if let Some((obj, t)) = hit {
                if t >= MAX_T {
                    return scene.background.color(&ray);
                }
                if t <= MIN_T {
                    return obj.material.albedo;
                }
                ray.origin = ray.at(t);
            } else {
                return scene.background.color(&ray);
            }
        }
    }
}
