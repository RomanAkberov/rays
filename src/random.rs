use crate::math::{Float, Vector2, Vector3};
use crate::renderer::Pixel;

pub struct Random {
    state: u32,
}

impl Random {
    pub fn from_seed(seed: u32) -> Self {
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

    pub fn range01<F: Float>(&mut self) -> F {
        F::of(self.next_state() as f64 / 4294967296.0)
    }

    pub fn in_sphere<F: Float>(&mut self) -> Vector3<F> {
        loop {
            let v = Vector3::new(
                self.range01::<F>() * F::TWO - F::ONE, 
                self.range01::<F>() * F::TWO - F::ONE,
                self.range01::<F>() * F::TWO - F::ONE,
            );
            if v.length_squared() < F::ONE {
                return v;
            }
        }
    }

    pub fn in_hemisphere<F: Float>(&mut self, normal: Vector3<F>) -> Vector3<F> {
        let in_sphere = self.in_sphere();
        if in_sphere.dot(normal) > F::ZERO {
            in_sphere
        } else {
            -in_sphere
        }
    }

    pub fn in_pixel<F: Float>(&mut self, pixel: Pixel<F>) -> Vector2<F> {
        Vector2 {
            x: (pixel.coord.x + self.range01::<F>()) / (pixel.frame_size.x - F::ONE),
            y: F::ONE - (pixel.coord.y + self.range01::<F>()) / (pixel.frame_size.y - F::ONE),
        }
    }
}
