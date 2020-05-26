use smth::Vec3;
use crate::{
    background::Background,
    object::Object,
};

pub struct SceneDef<S> {
    pub camera: CameraDef<S>,
    pub background: Background<S>,
    pub objects: Vec<Object<S>>,
}

pub struct CameraDef<S> {
    pub eye: Vec3<S>,
    pub target: Vec3<S>,
    pub fov: S,
    pub aperture: S,
}
