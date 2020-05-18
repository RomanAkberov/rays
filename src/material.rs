use serde::{Serialize, Deserialize};
use crate::{
    color::Color,
    math::{Float, Ray, Vector3},
    random::Random,
    shapes::Hit,
};

#[derive(Serialize, Deserialize)]
pub enum Mode<F> {
    Diffuse,
    Metallic(F),
    Dielectric(F),
}

#[derive(Serialize, Deserialize)]
pub struct Material<F> {
    pub mode: Mode<F>,
    pub albedo: Color<F>,
}

impl<F: Float> Material<F> {
    pub fn scatter(&self, ray: &Ray<F>, hit: &Hit<F>, random: &mut Random) -> Option<(Color<F>, Ray<F>)> {
        let origin = ray.at(hit.t);
        let (albedo, direction) = match self.mode {
            Mode::Diffuse => {
                (self.albedo, random.in_hemisphere(hit.normal))//hit.normal + random.in_sphere();
            }
            Mode::Metallic(fuzziness) => {
                let reflected = reflect(ray.direction.normalized(), hit.normal);
                let direction = if fuzziness > F::ZERO {
                    reflected + random.in_sphere() * fuzziness
                } else {
                    reflected
                };
                (self.albedo, direction)
            }
            Mode::Dielectric(reflection_index) => {
                let (normal, eta) = if ray.direction.dot(hit.normal) < F::ZERO {
                    (hit.normal, F::ONE / reflection_index)
                } else {
                    (-hit.normal, reflection_index)
                };
                // TODO do not compute cos_theta twice (here and in refract)?
                let cos_theta = ray.direction.dot(-normal).min(F::ONE);
                let sin_theta = (F::ONE - cos_theta * cos_theta).sqrt();
                if eta * sin_theta > F::ONE || random.probability(schlick(cos_theta, eta)) {
                    (Color::WHITE, reflect(ray.direction, normal))
                } else {
                    (Color::WHITE, refract(ray.direction, normal, eta)) 
                }
            }
        };
        let ray = Ray { origin, direction: direction.normalized() };
        Some((albedo, ray))
    }
}

fn schlick<F: Float>(cosine: F, eta: F) -> F {
    let mut r0 = (F::ONE - eta) / (F::ONE + eta);
    r0 *= r0;
    return r0 + (F::ONE - r0) * (F::ONE - cosine).powi(5)
}

fn reflect<F: Float>(vector: Vector3<F>, normal: Vector3<F>) -> Vector3<F> {
    vector - normal * F::TWO * vector.dot(normal)
}

fn refract<F: Float>(vector: Vector3<F>, normal: Vector3<F>, eta: F) -> Vector3<F> {
    let cos_theta = normal.dot(-vector);
    let parallel = (vector + normal * cos_theta) * eta;
    let perp = normal * -(F::ONE - parallel.length_squared()).sqrt();
    parallel + perp
}