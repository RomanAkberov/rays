use smth::{Float, Vec2};
use crate::{
    color::Color,
    config::ImageConfig,
    image::Image,
    progress::Progress,
    random::Random,
    scene::Scene,
};

#[derive(Copy, Clone)]
pub struct Pixel<S> {
    pub coord: Vec2<S>,
    pub frame_size: Vec2<S>,
}

pub trait PixelRenderer: Sync + Send {
    fn render_pixel<S: Float>(&self, scene: &Scene<S>, pixel: Pixel<S>, random: &mut Random) -> Color<S>;
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
    fn render_pixel<S: Float>(&self, scene: &Scene<S>, pixel: Pixel<S>, random: &mut Random) -> Color<S> {
        let scale = S::of(1.0 / self.samples as f64);
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
    pub fn render<S: Float + Send + Sync>(&mut self, scene: &Scene<S>, config: &ImageConfig, progress: &dyn Progress) -> Image<S> {
        use rayon::prelude::*;

        let width = config.width;
        let height = config.height;
        let frame_size = Vec2::new(S::of(width as f64), S::of(height as f64));
        let colors: Vec<Vec<Color<S>>> = (0 .. height)
            .map(|j| (j, Random::new(42 + j)))
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(j, mut random)| {
                let mut colors = Vec::with_capacity(width as usize);
                for i in 0 .. width {
                    let pixel = Pixel {
                        coord: Vec2::new(S::of(i as f64), S::of(j as f64)),
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
