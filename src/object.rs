use serde::Deserialize;
use crate::{
    shapes::Sphere,
    material::Material,
};

#[derive(Deserialize)]
pub struct Object {
    pub shape: Sphere,
    pub material: Material,
}
