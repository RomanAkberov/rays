use serde::Deserialize;
use crate::{
    background::Background,
    math::{Float, Vec3},
    object::Object,
};

#[derive(Deserialize)]
pub struct SceneDef {
    pub camera: CameraDef,
    pub background: Background,
    pub objects: Vec<Object>,
}

#[derive(Deserialize)]
pub struct CameraDef {
    pub eye: Vec3,
    pub target: Vec3,
    pub fov: Float,
    pub aperture: Float,
}
