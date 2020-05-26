use crate::{
    material::Material,
    math::Shape,
};

pub struct Object<S> {
    pub shape: Shape<S>,
    pub material: Material<S>,
}
