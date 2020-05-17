use serde::Deserialize;
use crate::{
    shapes::Sphere,
    material::Material,
};

#[derive(Deserialize)]
pub struct Object<F> {
    pub shape: Sphere<F>,
    pub material: Material<F>,
}
