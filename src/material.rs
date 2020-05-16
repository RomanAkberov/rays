use serde::{Serialize, Deserialize};
use crate::{
    color::Color,
    math::Ray,
    random::Random,
    shapes::Hit,
};

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub albedo: Color,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)> {
        let origin = ray.at(hit.t);
        let direction = random.in_hemisphere(&hit.normal);//hit.normal + random.in_sphere();
        let ray = Ray { origin, direction };
        Some((self.albedo, ray))
    }
}
