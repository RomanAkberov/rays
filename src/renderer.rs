use crate::{
    color::Color,
    config::ImageConfig,
    math::Vector2,
    image::Image,
    scene::Scene,
};

#[derive(Copy, Clone)]
pub struct Pixel {
    pub coord: Vector2,
    pub frame_size: Vector2,
}

pub trait PixelRenderer {
    fn render_pixel(&mut self, scene: &Scene, pixel: Pixel) -> Color;
}

pub struct Supersampler<P: PixelRenderer> {
    inner: P,
    samples: u32,
}

impl<P: PixelRenderer> Supersampler<P> {
    pub fn new(inner: P, samples: u32) -> Self {
        Self { inner, samples }
    }
}

impl<P: PixelRenderer> PixelRenderer for Supersampler<P> {
    fn render_pixel(&mut self, scene: &Scene, pixel: Pixel) -> Color {
        let scale = 1.0 / self.samples as f64;
        let mut color = Color::BLACK;
        for _ in 0 .. self.samples {
            color += self.inner.render_pixel(scene, pixel);
        }
        color * scale
    }
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
        let frame_size = Vector2::new(width as f64, height as f64);
        let mut colors = Vec::with_capacity((width * height) as usize);
        for j in 0 .. height {
            for i in 0 .. width {
                let pixel = Pixel {
                    coord: Vector2::new(i as f64, j as f64),
                    frame_size,
                };
                let mut color = self.pixel_renderer.render_pixel(scene, pixel);
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
