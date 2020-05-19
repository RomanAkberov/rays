use super::{Float, Vec3};

pub struct Hit {
    pub t: Float,
    pub point: Vec3,
    pub normal: Vec3,
}
