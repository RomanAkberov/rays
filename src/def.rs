use smth::Vec3D;
use crate::{
    background::Background,
    math::Float,
    object::Object,
};

pub struct SceneDef {
    pub camera: CameraDef,
    pub background: Background,
    pub objects: Vec<Object>,
}

pub struct CameraDef {
    pub eye: Vec3D,
    pub target: Vec3D,
    pub fov: Float,
    pub aperture: Float,
}
