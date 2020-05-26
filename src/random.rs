use smth::{Float, Vec2, Vec3};
use crate::renderer::Pixel;

pub struct Random {
    state: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        let state = if seed != 0 {
            seed
        } else {
            42
        };
        Self { state }
    }

    pub fn next_state(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    pub fn in_01(&mut self) -> f64 {
        self.next_state() as f64 / 4294967296.0
    }

    pub fn in_range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.in_01()
    }

    pub fn probability<S: Float>(&mut self, probability: S) -> bool {
        S::of(self.in_01()) < probability
    }

    pub fn in_sphere<S: Float>(&mut self) -> Vec3<S> {
        loop {
            let v = Vec3 {
                x: S::of(self.in_range(-1.0, 1.0)), 
                y: S::of(self.in_range(-1.0, 1.0)),
                z: S::of(self.in_range(-1.0, 1.0)),
            };
            if v.len2() < S::ONE {
                return v;
            }
        }
    }

    pub fn in_hemisphere<S: Float>(&mut self, normal: Vec3<S>) -> Vec3<S> {
        let in_sphere = self.in_sphere();
        if in_sphere.dot(normal) > S::ZERO {
            in_sphere
        } else {
            -in_sphere
        }
    }

    pub fn in_disk<S: Float>(&mut self) -> Vec2<S> {
        loop {
            let v = Vec2 {
                x: S::of(self.in_range(-1.0, 1.0)), 
                y: S::of(self.in_range(-1.0, 1.0)), 
            };
            if v.len2() < S::ONE {
                return v;
            }
        }
    }

    pub fn in_pixel<S: Float>(&mut self, pixel: Pixel<S>) -> Vec2<S> {
        Vec2 {
            x: (pixel.coord.x + S::of(self.in_01())) / (pixel.frame_size.x - S::ONE),
            y: S::ONE - (pixel.coord.y + S::of(self.in_01())) / (pixel.frame_size.y - S::ONE),
        }
    }
}
