use smth::Vec3;

pub struct Hit<S> {
    pub t: S,
    pub point: Vec3<S>,
    pub normal: Vec3<S>,
}
