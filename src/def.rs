use serde::Deserialize;
use crate::{
    background::Background,
    math::Vector3,
    object::Object,
};

#[derive(Deserialize)]
pub struct SceneDef<F> {
    pub camera: CameraDef<F>,
    pub background: Background<F>,
    pub objects: Vec<Object<F>>,
}

#[derive(Deserialize)]
pub struct CameraDef<F> {
    pub eye: Vector3<F>,
    pub target: Vector3<F>,
    pub fov: F,
    pub aperture: F,
}
