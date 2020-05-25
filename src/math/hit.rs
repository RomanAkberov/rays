use smth::Vec3D;
use super::Float;

pub struct Hit {
    pub t: Float,
    pub point: Vec3D,
    pub normal: Vec3D,
}
