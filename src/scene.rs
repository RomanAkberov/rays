use crate::{
    background::Background,
    camera::Camera,
    def::SceneDef,
    math::{Float, Vec3},
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
                Vec3::new(0.0, 1.0, 0.0),
                def.camera.fov, 
                aspect,
                def.camera.aperture,
            ),
            background: def.background,
            objects: def.objects,
        }
    }
}
