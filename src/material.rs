use serde::{Serialize, Deserialize};
use crate::{
    color::Color,
    math::{Float, Ray, Vec3},
    random::Random,
    shapes::Hit,
};

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Diffuse,
    Metallic(Float),
    Dielectric(Float),
}

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub mode: Mode,
    pub albedo: Color,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)> {
        let origin = ray.at(hit.t);
        let (albedo, direction) = match self.mode {
            Mode::Diffuse => {
                (self.albedo, random.in_hemisphere(hit.normal))//hit.normal + random.in_sphere();
            }
            Mode::Metallic(fuzziness) => {
                let reflected = reflect(ray.direction.normalized(), hit.normal);
                let direction = if fuzziness > 0.0 {
                    reflected + random.in_sphere() * fuzziness
                } else {
                    reflected
                };
                (self.albedo, direction)
            }
            Mode::Dielectric(reflection_index) => {
                let (normal, eta) = if ray.direction.dot(hit.normal) < 0.0 {
                    (hit.normal, 1.0 / reflection_index)
                } else {
                    (-hit.normal, reflection_index)
                };
                // TODO do not compute cos_theta twice (here and in refract)?
                let cos_theta = ray.direction.dot(-normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                if eta * sin_theta > 1.0 || random.probability(schlick(cos_theta, eta)) {
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

fn schlick(cosine: Float, eta: Float) -> Float {
    let mut r0 = (1.0 - eta) / (1.0 + eta);
    r0 *= r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - normal * 2.0 * vector.dot(normal)
}

fn refract(vector: Vec3, normal: Vec3, eta: Float) -> Vec3 {
    let cos_theta = normal.dot(-vector);
    let parallel = (vector + normal * cos_theta) * eta;
    let perp = normal * -(1.0 - parallel.len2()).sqrt();
    parallel + perp
}