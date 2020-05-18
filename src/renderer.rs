use crate::{
    color::Color,
    config::ImageConfig,
    math::{Float, Vec2},
    image::Image,
    progress::Progress,
    random::Random,
    scene::Scene,
};

#[derive(Copy, Clone)]
pub struct Pixel {
    pub coord: Vec2,
    pub frame_size: Vec2,
}

pub trait PixelRenderer: Sync + Send {
    fn render_pixel(&self, scene: &Scene, pixel: Pixel, random: &mut Random) -> Color;
}

pub struct Multisampler<P: PixelRenderer> {
    inner: P,
    samples: u32,
}

impl<P: PixelRenderer> Multisampler<P> {
    pub fn new(inner: P, samples: u32) -> Self {
        Self { inner, samples }
    }
}

impl<P: PixelRenderer> PixelRenderer for Multisampler<P> {
    fn render_pixel(&self, scene: &Scene, pixel: Pixel, random: &mut Random) -> Color {
        let scale = 1.0 / self.samples as Float;
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

impl<P: PixelRenderer> Renderer<P> {
    pub fn render(&mut self, scene: &Scene, config: &ImageConfig, progress: &dyn Progress) -> Image {
        use rayon::prelude::*;

        let width = config.width;
        let height = config.height;
        let frame_size = Vec2::new(width as Float, height as Float);
        let colors: Vec<Vec<Color>> = (0 .. height)
            .map(|j| (j, Random::new(42 + j)))
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(j, mut random)| {
                let mut colors = Vec::with_capacity(width as usize);
                for i in 0 .. width {
                    let pixel = Pixel {
                        coord: Vec2::new(i as Float, j as Float),
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
                progress.increment(height);
                colors
            })
            .collect();
        Image { 
            width, height, 
            colors: colors.iter().flatten().copied().collect() 
        }
    }
}
