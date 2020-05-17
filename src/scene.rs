use crate::{
    background::Background,
    camera::Camera,
    def::SceneDef,
    math::{Float, Vector3},
    object::Object,
};

pub struct Scene<F> {
    pub camera: Camera<F>,
    pub background: Background<F>,
    pub objects: Vec<Object<F>>,
}

impl<F: Float> Scene<F> {
    pub fn load(def: SceneDef<F>, aspect: F) -> Self {
        Self {
            camera: Camera::new(
                def.camera.eye, 
                def.camera.target,
                Vector3::new(F::ZERO, F::ONE, F::ZERO),
                def.camera.fov, 
                aspect
            ),
            background: def.background,
            objects: def.objects,
        }
    }
}
