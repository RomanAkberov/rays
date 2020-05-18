use crate::{
    shapes::Shape,
    material::Material,
};

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub material: Box<dyn Material>,
}
