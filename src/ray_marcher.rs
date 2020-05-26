use smth::Float;
use crate::{
    color::Color,
    random::Random,
    renderer::{PixelRenderer, Pixel},
    scene::Scene,
};

pub struct RayMarcher {

}

impl RayMarcher {
    pub fn new() -> Self {
        Self { }
    }
}

impl PixelRenderer for RayMarcher {
    fn render_pixel<S: Float>(&self, scene: &Scene<S>, pixel: Pixel<S>, random: &mut Random) -> Color<S> {
        let t_min = S::of(0.0001);
        let t_max = S::of(100.0);
        let uv = random.in_pixel(pixel);
        let mut ray = scene.camera.ray(uv, random);
        loop {
            let hit = scene.objects
                .iter()
                .map(|obj| (&obj.material, obj.shape.sdf(ray.origin)))
                .min_by(|hit1, hit2| hit1.1.partial_cmp(&hit2.1).unwrap());
            if let Some((material, t)) = hit {
                if t > t_max {
                    return scene.background.color(&ray);
                }
                if t < t_min {
                    return material.color();
                }
                ray.origin = ray.at(t);
            } else {
                return scene.background.color(&ray);
            }
        }
    }
}
