use serde::Deserialize;
use crate::{
    background::Background,
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
    pub fov: F,
}
