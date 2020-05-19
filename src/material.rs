use crate::{
    color::Color,
    math::{Float, Hit, Ray, Vec3},
    random::Random,
    texture::Texture,
};

pub enum Material {
    Diffuse(Texture),
    Metallic(Color, Float),
    Dielectric(Float),
}

impl Material {
    pub fn color(&self) -> Color {
        match self {
            Self::Diffuse(_) => Color::WHITE,
            &Self::Metallic(color, _) => color,
            Self::Dielectric(_) => Color::WHITE,
        }
    }

    pub fn scatter(&self, ray: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)> {
        let (color, direction) = match self {
            Self::Diffuse(texture) => {
                let direction = random.in_hemisphere(hit.normal); // hit.normal + random.in_sphere();
                (texture.color(hit), direction)
            }
            &Self::Metallic(color, fuzziness) => {
                let reflected = reflect(ray.direction.normalized(), hit.normal);
                let direction = if fuzziness > 0.0 {
                    reflected + random.in_sphere() * fuzziness
                } else {
                    reflected
                };
                (color, direction)
            }
            &Self::Dielectric(index) => {
                let (normal, eta) = if ray.direction.dot(hit.normal) < 0.0 {
                    (hit.normal, 1.0 / index)
                } else {
                    (-hit.normal, index)
                };
                // TODO do not compute cos_theta twice (here and in refract)?
                let cos_theta = ray.direction.dot(-normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let direction = if eta * sin_theta > 1.0 || random.probability(schlick(cos_theta, eta)) {
                    reflect(ray.direction, normal)
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
