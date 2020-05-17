use serde::{Serialize, Deserialize};
use crate::{
    color::Color,
    math::{Ray, Vector3},
    random::Random,
    shapes::Hit,
};

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Diffuse,
    Metallic,
}

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub albedo: Color,
    pub mode: Mode,
    #[serde(default)]
    pub fuzziness: f64,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)> {
        let origin = ray.at(hit.t);
        let direction = match self.mode {
            Mode::Diffuse => {
                random.in_hemisphere(hit.normal)//hit.normal + random.in_sphere();
            }
            Mode::Metallic => {
                let reflected = reflect(ray.direction.normalized(), hit.normal);
                if self.fuzziness > 0.0 {
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

fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
    vector - normal * 2.0 * vector.dot(normal)
}