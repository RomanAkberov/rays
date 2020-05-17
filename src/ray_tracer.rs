use crate::{
    color::Color,
    math::Ray,
    random::Random,
    renderer::{PixelRenderer, Pixel},
    scene::Scene,
};

pub struct RayTracer {
    max_depth: u32,
    random: Random,
}

impl RayTracer {
    pub fn new(max_depth: u32) -> Self {
        Self {
            max_depth,
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
        scene.background.color(ray)
    }
}

impl PixelRenderer for RayTracer {
    fn render_pixel(&mut self, scene: &Scene, pixel: Pixel) -> Color {
        let ray = scene.camera.ray(self.random.in_pixel(pixel));
        self.cast_ray(&ray, scene, self.max_depth)
    }
}
