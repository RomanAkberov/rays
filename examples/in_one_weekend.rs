fn main() -> rays::RayResult<()> {
    let config = rays::Config {
        image: rays::ImageConfig {
            path: "ray-tracer.png".to_string(),
            width: 1920,
            height: 1080,
            gamma_correction: true,
        },
        precision: rays::Precision::F64,
        max_depth: 50,
        samples: 1024,
        renderer: rays::RenderMode::RayTracer,
        show_progress: true,
    };
    rays::run_loader(&config, Loader)
} 

pub struct Loader;

impl rays::SceneLoader for Loader {
    fn load<F: rays::Float>(self) -> rays::RayResult<rays::SceneDef<F>> {
        let mut objects = Vec::new();
        let ground_material = rays::Material {
            mode: rays::Mode::Diffuse,
            albedo: rays::Color::new(F::of(0.5), F::of(0.5), F::of(0.5)),
        };
        objects.push(rays::Object {
            shape: rays::Sphere {
                radius: F::of(1000.0),
                center: rays::Vector3::new(F::of(0.0), F::of(-1000.0), F::of(0.0))
            },
            material: ground_material,
        });
        let mut random = rays::Random::from_seed(42);
        for a in -11 .. 11 {
            for b in -11 .. 11 {
                let choose_mat = random.range01::<F>();
                let center = rays::Vector3::new(
                    F::of(a as f64) + F::of(0.9) * random.range01(), 
                    F::of(0.2),
                    F::of(b as f64) + F::of(0.9) * random.range01(),
                );
                if center.distance(rays::Vector3::new(F::of(4.0), F::of(0.2), F::of(0.0))) > F::of(0.9) {
                    let material = if choose_mat < F::of(0.8) {
                        // diffuse
                        rays::Material {
                            mode: rays::Mode::Diffuse,
                            albedo: rays::Color {
                                r: random.range01::<F>() * random.range01::<F>(),
                                g: random.range01::<F>() * random.range01::<F>(),
                                b: random.range01::<F>() * random.range01::<F>(),
                            },
                        }
                    } else if choose_mat < F::of(0.95) {
                        // metal
                        rays::Material {
                            mode: rays::Mode::Metallic(random.range01::<F>() * F::HALF),
                            albedo: rays::Color {
                                r: random.range01::<F>() * F::HALF + F::HALF,
                                g: random.range01::<F>() * F::HALF + F::HALF,
                                b: random.range01::<F>() * F::HALF + F::HALF,
                            },
                        }
                    } else {
                        rays::Material {
                            mode: rays::Mode::Dielectric(F::of(1.5)),
                            albedo: rays::Color::WHITE,
                        }
                    };
                    objects.push(rays::Object {
                        shape: rays::Sphere {
                            radius: F::of(0.2),
                            center,
                        },
                        material,
                    });
                }
            }
        }
        objects.push(rays::Object {
            shape: rays::Sphere {
                radius: F::ONE,
                center: rays::Vector3::new(F::ZERO, F::ONE, F::ZERO),
            },
            material: rays::Material {
                mode: rays::Mode::Dielectric(F::of(1.5)),
                albedo: rays::Color::WHITE,
            }
        });
        objects.push(rays::Object {
            shape: rays::Sphere {
                radius: F::ONE,
                center: rays::Vector3::new(F::of(-4.0), F::ONE, F::ZERO),
            },
            material: rays::Material {
                mode: rays::Mode::Diffuse,
                albedo: rays::Color::new(F::of(0.4), F::of(0.2), F::of(0.1)),
            }
        });
        objects.push(rays::Object {
            shape: rays::Sphere {
                radius: F::ONE,
                center: rays::Vector3::new(F::of(4.0), F::ONE, F::ZERO),
            },
            material: rays::Material {
                mode: rays::Mode::Metallic(F::ZERO),
                albedo: rays::Color::new(F::of(0.7), F::of(0.6), F::of(0.5)),
            }
        });
        Ok(rays::SceneDef {
            camera: rays::CameraDef {
                eye: rays::Vector3::new(F::of(13.0), F::of(2.0), F::of(3.0)),
                target: rays::Vector3::ZERO,
                fov: F::of(20.0),
                aperture: F::of(0.2),
            },
            background: rays::Background {
                top: rays::Color::new(F::of(0.5), F::of(0.7), F::of(1.0)),
                bottom: rays::Color::new(F::of(1.0), F::of(1.0), F::of(1.0)),
            },
            objects,
        })
    }
}
