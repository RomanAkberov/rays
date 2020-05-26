use smth::{Float, Vec3};
use crate::{
    color::Color,
    math::{Hit, Ray},
    random::Random,
    texture::Texture,
};

pub enum Material<S> {
    Diffuse(Texture<S>),
    Metallic(Color<S>, S),
    Dielectric(S),
}

impl<S: Float> Material<S> {
    pub fn color(&self) -> Color<S> {
        match self {
            Self::Diffuse(_) => Color::WHITE,
            &Self::Metallic(color, _) => color,
            Self::Dielectric(_) => Color::WHITE,
        }
    }

    pub fn scatter(&self, ray: &Ray<S>, hit: &Hit<S>, random: &mut Random) -> Option<(Color<S>, Ray<S>)> {
        let (color, direction) = match self {
            Self::Diffuse(texture) => {
                let direction = random.in_hemisphere(hit.normal); // hit.normal + random.in_sphere();
                (texture.color(hit), direction)
            }
            &Self::Metallic(color, fuzziness) => {
                let reflected = ray.direction.reflect(hit.normal);
                let direction = if fuzziness > S::ZERO {
                    reflected + random.in_sphere() * fuzziness
                } else {
                    reflected
                };
                (color, direction)
            }
            &Self::Dielectric(index) => {
                let (normal, eta) = if ray.direction.dot(hit.normal) < S::ZERO {
                    (hit.normal, S::ONE / index)
                } else {
                    (-hit.normal, index)
                };
                // TODO do not compute cos_theta twice (here and in refract)?
                let mut cos_theta = ray.direction.dot(-normal);
                if cos_theta > S::ONE {
                    cos_theta = S::ONE;
                }
                let sin_theta = (S::ONE - cos_theta * cos_theta).sqrt();
                let direction = if eta * sin_theta > S::ONE || random.probability(schlick(cos_theta, eta)) {
                    ray.direction.reflect(normal)
                } else {
                    refract(ray.direction, normal, eta)
                };
                (Color::WHITE, direction)
            }
        };
        let ray = Ray {
            origin: hit.point,
            direction: direction.normalized()
        };
        Some((color, ray))
    }
}

fn schlick<S: Float>(cosine: S, eta: S) -> S {
    let mut r0 = (S::ONE - eta) / (S::ONE + eta);
    r0 *= r0;
    return r0 + (S::ONE - r0) * (S::ONE - cosine).powi(5)
}

fn refract<S: Float>(vector: Vec3<S>, normal: Vec3<S>, eta: S) -> Vec3<S> {
    let cos_theta = normal.dot(-vector);
    let parallel = (vector + normal * cos_theta) * eta;
    let perp = normal * -(S::ONE - parallel.len2()).sqrt();
    parallel + perp
}
