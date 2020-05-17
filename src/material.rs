use serde::{Serialize, Deserialize};
use crate::{
    color::Color,
    math::{Float, Ray, Vector3},
    random::Random,
    shapes::Hit,
};

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Diffuse,
    Metallic,
}

#[derive(Serialize, Deserialize)]
pub struct Material<F> {
    pub albedo: Color<F>,
    pub mode: Mode,
    pub fuzziness: F,
}

impl<F: Float> Material<F> {
    pub fn scatter(&self, ray: &Ray<F>, hit: &Hit<F>, random: &mut Random) -> Option<(Color<F>, Ray<F>)> {
        let origin = ray.at(hit.t);
        let direction = match self.mode {
            Mode::Diffuse => {
                random.in_hemisphere(hit.normal)//hit.normal + random.in_sphere();
            }
            Mode::Metallic => {
                let reflected = reflect(ray.direction.normalized(), hit.normal);
                if self.fuzziness > F::ZERO {
                    reflected + random.in_sphere() * self.fuzziness
                } else {
                    reflected
                }
            }
        };
        let ray = Ray { origin, direction: direction.normalized() };
        Some((self.albedo, ray))
    }
}

fn reflect<F: Float>(vector: Vector3<F>, normal: Vector3<F>) -> Vector3<F> {
    vector - normal * F::TWO * vector.dot(normal)
}