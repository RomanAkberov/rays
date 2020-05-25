use smth::Vec3D;
use crate::{
    background::Background,
    camera::Camera,
    def::SceneDef,
    math::{Float},
    object::Object,
};

pub struct Scene {
    pub camera: Camera,
    pub background: Background,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn load(def: SceneDef, aspect: Float) -> Self {
        Self {
            camera: Camera::new(
                def.camera.eye, 
                def.camera.target,
                Vec3D::Y,
                def.camera.fov, 
                aspect,
                def.camera.aperture,
            ),
            background: def.background,
            objects: def.objects,
        }
    }
}
