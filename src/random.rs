use crate::math::{Float, Vec2, Vec3};
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

    pub fn range01(&mut self) -> Float {
        self.next_state() as Float / 4294967296.0
    }

    pub fn in_range(&mut self, min: Float, max: Float) -> Float {
        min + (max - min) * self.range01()
    }

    pub fn probability(&mut self, probability: Float) -> bool {
        self.range01() < probability
    }

    pub fn in_sphere(&mut self) -> Vec3 {
        loop {
            let v = Vec3::new(
                self.in_range(-1.0, 1.0), 
                self.in_range(-1.0, 1.0),
                self.in_range(-1.0, 1.0),
            );
            if v.len2() < 1.0 {
                return v;
            }
        }
    }

    pub fn in_hemisphere(&mut self, normal: Vec3) -> Vec3 {
        let in_sphere = self.in_sphere();
        if in_sphere.dot(normal) > 0.0 {
            in_sphere
        } else {
            -in_sphere
        }
    }

    pub fn in_disk(&mut self) -> Vec2 {
        loop {
            let v = Vec2::new(
                self.in_range(-1.0, 1.0), 
                self.in_range(-1.0, 1.0), 
            );
            if v.len2() < 1.0 {
                return v;
            }
        }
    }

    pub fn in_pixel(&mut self, pixel: Pixel) -> Vec2 {
        Vec2 {
            x: (pixel.coord.x + self.range01()) / (pixel.frame_size.x - 1.0),
            y: 1.0 - (pixel.coord.y + self.range01()) / (pixel.frame_size.y - 1.0),
        }
    }
}
