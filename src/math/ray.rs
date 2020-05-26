use smth::{Float, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray<S> {
    pub origin: Vec3<S>,
    pub direction: Vec3<S>,
}

impl<S: Float> Ray<S> {
    pub fn at(&self, t: S) -> Vec3<S> {
        self.origin + self.direction * t
    }
}
