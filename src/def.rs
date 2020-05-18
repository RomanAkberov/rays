use crate::{
    background::Background,
    math::{Float, Vec3},
    object::Object,
};

pub struct SceneDef {
    pub camera: CameraDef,
    pub background: Background,
    pub objects: Vec<Object>,
}

pub struct CameraDef {
    pub eye: Vec3,
    pub target: Vec3,
    pub fov: Float,
    pub aperture: Float,
}
