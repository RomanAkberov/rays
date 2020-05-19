use crate::{
    material::Material,
    math::Shape,
};

pub struct Object {
    pub shape: Shape,
    pub material: Material,
}
