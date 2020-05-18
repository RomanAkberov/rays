use crate::{
    color::Color,
    math::Float,
    random::Random,
    renderer::{PixelRenderer, Pixel},
    scene::Scene,
};

pub const T_MIN: Float = 0.0001;
pub const T_MAX: Float = 100.0;

pub struct RayMarcher {

}

impl RayMarcher {
    pub fn new() -> Self {
        Self { }
    }
}

impl PixelRenderer for RayMarcher {
    fn render_pixel(&self, scene: &Scene, pixel: Pixel, random: &mut Random) -> Color {
        let uv = random.in_pixel(pixel);
        let mut ray = scene.camera.ray(uv, random);
        loop {
            let hit = scene.objects
                .iter()
                .map(|obj| (obj, obj.shape.sdf(ray.origin)))
                .min_by(|hit1, hit2| hit1.1.partial_cmp(&hit2.1).unwrap());
            if let Some((obj, t)) = hit {
                if t > T_MAX {
                    return scene.background.color(&ray);
                }
                if t < T_MIN {
                    return obj.material.albedo;
                }
                ray.origin = ray.at(t);
            } else {
                return scene.background.color(&ray);
            }
        }
    }
}
