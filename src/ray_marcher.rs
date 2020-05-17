use crate::{
    color::Color,
    renderer::PixelRenderer,
    scene::Scene,
};

pub const MIN_T: f64 = 0.0001;
pub const MAX_T: f64 = 100.0;

pub struct RayMarcher {

}

impl RayMarcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl PixelRenderer for RayMarcher {
    fn render_pixel(&mut self, scene: &Scene, pixel: (u32, u32), size: (u32, u32)) -> Color {
        let u = (pixel.0 as f64) / (size.0 - 1) as f64;
        let v = 1.0 - (pixel.1 as f64) / (size.1 - 1) as f64;
        let mut ray = scene.camera.ray(u, v);
        loop {
            let hit = scene.objects
                .iter()
                .map(|obj| (obj, obj.shape.distance(ray.origin)))
                .min_by(|hit1, hit2| hit1.1.partial_cmp(&hit2.1).unwrap());
            if let Some((obj, t)) = hit {
                if t >= MAX_T {
                    return Color::BLACK;
                }
                if t <= MIN_T {
                    return obj.material.albedo;
                }
                ray.origin = ray.at(t);
            } else {
                return Color::BLACK;
            }
        }
    }
}
