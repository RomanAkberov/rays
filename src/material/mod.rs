use crate::{
    color::Color,
    math::{Float, Ray, Vec3},
    random::Random,
    shapes::Hit,
    texture::Texture,
};

pub trait Material: Sync {
    fn color(&self) -> Color;
    fn scatter(&self, ray: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)>;
}

pub struct Diffuse {
    pub texture: Box<dyn Texture>,
}

impl Material for Diffuse {
    fn color(&self) -> Color {
        Color::WHITE
    }

    fn scatter(&self, _: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)> {
        let direction = random.in_hemisphere(hit.normal); // hit.normal + random.in_sphere();
        let ray = Ray {
            origin: hit.point,
            direction: direction.normalized(),
        };
        Some((self.texture.value(hit), ray))
    }
}

pub struct Metallic {
    pub color: Color,
    pub fuzziness: Float,
}

impl Material for Metallic {
    fn color(&self) -> Color {
        self.color
    }

    fn scatter(&self, ray: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)> {
        let reflected = reflect(ray.direction.normalized(), hit.normal);
        let direction = if self.fuzziness > 0.0 {
            reflected + random.in_sphere() * self.fuzziness
        } else {
            reflected
        };
        let ray = Ray {
            origin: hit.point,
            direction: direction.normalized(),
        };
        Some((self.color, ray))
    }
}

pub struct Dielectric {
    pub index: Float,
}

impl Material for Dielectric {
    fn color(&self) -> Color {
        Color::WHITE
    }

    fn scatter(&self, ray: &Ray, hit: &Hit, random: &mut Random) -> Option<(Color, Ray)> {
        let (normal, eta) = if ray.direction.dot(hit.normal) < 0.0 {
            (hit.normal, 1.0 / self.index)
        } else {
            (-hit.normal, self.index)
        };
        // TODO do not compute cos_theta twice (here and in refract)?
        let cos_theta = ray.direction.dot(-normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction = if eta * sin_theta > 1.0 || random.probability(schlick(cos_theta, eta)) {
            reflect(ray.direction, normal)
        } else {
            refract(ray.direction, normal, eta)
        };
        let ray = Ray {
            origin: hit.point,
            direction: direction.normalized(),
        };
        Some((Color::WHITE, ray))
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