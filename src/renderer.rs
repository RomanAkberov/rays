use crate::{
    color::Color,
    config::ImageConfig,
    math::Vector2,
    image::Image,
    random::Random,
    scene::Scene,
};

#[derive(Copy, Clone)]
pub struct Pixel {
    pub coord: Vector2,
    pub frame_size: Vector2,
}

pub trait PixelRenderer: Sync + Send {
    fn render_pixel(&self, scene: &Scene, pixel: Pixel, random: &mut Random) -> Color;
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
    fn render_pixel(&self, scene: &Scene, pixel: Pixel, random: &mut Random) -> Color {
        let scale = 1.0 / self.samples as f64;
        let mut color = Color::BLACK;
        for _ in 0 .. self.samples {
            color += self.inner.render_pixel(scene, pixel, random);
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

impl<P: PixelRenderer + Sync + Send> Renderer<P> {
    pub fn render(&mut self, scene: &Scene, config: &ImageConfig) -> Image {
        use rayon::prelude::*;

        let width = config.width;
        let height = config.height;
        let frame_size = Vector2::new(width as f64, height as f64);
        let colors: Vec<Vec<Color>> = (0 .. height)
            .map(|j| (j, Random::from_seed(42 + j)))
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(j, mut random)| {
                let mut colors = Vec::with_capacity(width as usize);
                for i in 0 .. width {
                    let pixel = Pixel {
                        coord: Vector2::new(i as f64, j as f64),
                        frame_size,
                    };
                    let mut color = self.pixel_renderer.render_pixel(scene, pixel, &mut random);
                    if config.gamma_correction {
                        color.r = color.r.sqrt();
                        color.g = color.g.sqrt();
                        color.b = color.b.sqrt();
                    }
                    colors.push(color);
                }
                colors
            })
            .collect();
        Image { 
            width, height, 
            colors: colors.iter().flatten().copied().collect() 
        }
    }
}
