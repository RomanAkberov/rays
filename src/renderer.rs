use crate::{
    color::Color,
    config::ImageConfig,
    image::Image,
    scene::Scene,
};

pub trait PixelRenderer {
    fn render_pixel(&mut self, scene: &Scene, pixel: (u32, u32), size: (u32, u32)) -> Color;
}

pub struct Renderer<P: PixelRenderer> {
    pixel_renderer: P,
}

impl<P: PixelRenderer> Renderer<P> {
    pub fn new(pixel_renderer: P) -> Self {
        Self { pixel_renderer }
    }
}

impl<P: PixelRenderer> Renderer<P> {
    pub fn render(&mut self, scene: &Scene, config: &ImageConfig) -> Image {
        let width = config.width;
        let height = config.height;
        let mut colors = Vec::with_capacity((width * height) as usize);
        for j in 0 .. height {
            for i in 0 .. width {
                let mut color = self.pixel_renderer.render_pixel(scene, (i, j), (width, height));
                if config.gamma_correction {
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
