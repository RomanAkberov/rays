use crate::{
    image::Image,
    scene::Scene,
};

pub trait Renderer {
    fn render(&mut self, scene: &Scene) -> Image;
}