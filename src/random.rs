use crate::math::Vector3;

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

    pub fn next_state(&mut self) {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
    }

    pub fn range01(&mut self) -> f64 {
        self.next_state();
        self.state as f64 / 4294967296.0
    }

    pub fn in_sphere(&mut self) -> Vector3 {
        loop {
            let v = Vector3::new(
                self.range01() * 2.0 - 1.0, 
                self.range01() * 2.0 - 1.0,
                self.range01() * 2.0 - 1.0,
            );
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn in_hemisphere(&mut self, normal: &Vector3) -> Vector3 {
        let in_sphere = self.in_sphere();
        if in_sphere.dot(normal) > 0.0 {
            in_sphere
        } else {
            -in_sphere
        }
    }
}
