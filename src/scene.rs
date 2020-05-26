use smth::{Float, Vec3};
use crate::{
    background::Background,
    camera::Camera,
    def::SceneDef,
    object::Object,
};

pub struct Scene<S> {
    pub camera: Camera<S>,
    pub background: Background<S>,
    pub objects: Vec<Object<S>>,
}

impl<S: Float> Scene<S> {
    pub fn load(def: SceneDef<S>, aspect: S) -> Self {
        Self {
            camera: Camera::new(
                def.camera.eye, 
                def.camera.target,
                Vec3::Y,
                def.camera.fov, 
                aspect,
                def.camera.aperture,
            ),
            background: def.background,
            objects: def.objects,
        }
    }
}
