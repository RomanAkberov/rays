use rays::*;

fn main() -> RayResult<()> {
    let config = Config {
        image: ImageConfig {
            path: "ray-tracer.png".to_string(),
            width: 1920,
            height: 1080,
            gamma_correction: true,
        },
        max_depth: 50,
        samples: 10,
        renderer: RenderMode::RayTracer,
        show_progress: true,
    };
    run_scene(&config, scene())
} 

fn scene() -> SceneDef {
    let mut objects = Vec::new();
    let ground_material = Material {
        mode: Mode::Diffuse,
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    objects.push(Object {
        shape: Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
        },
        material: ground_material,
    });
    let mut random = Random::new(42);
    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat = random.range01();
            let center = Vec3::new(
                a as Float + 0.9 * random.range01(), 
                0.2,
                b as Float + 0.9 * random.range01(),
            );
            if center.distance(Vec3::new(4.0, 0.2, 0.0)) > 0.9 {
                let material = if choose_mat < 0.8 {
                    // diffuse
                    Material {
                        mode: Mode::Diffuse,
                        albedo: Color {
                            r: random.range01() * random.range01(),
                            g: random.range01() * random.range01(),
                            b: random.range01() * random.range01(),
                        },
                    }
                } else if choose_mat < 0.95 {
                    // metal
                    Material {
                        mode: Mode::Metallic(random.in_range(0.0, 0.5)),
                        albedo: Color {
                            r: random.in_range(0.5, 1.0),
                            g: random.in_range(0.5, 1.0),
                            b: random.in_range(0.5, 1.0),
                        },
                    }
                } else {
                    Material {
                        mode: Mode::Dielectric(1.5),
                        albedo: Color::WHITE,
                    }
                };
                objects.push(Object {
                    shape: Sphere {
                        center,
                        radius: 0.2,
                    },
                    material,
                });
            }
        }
    }
    objects.push(Object {
        shape: Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
        },
        material: Material {
            mode: Mode::Dielectric(1.5),
            albedo: Color::WHITE,
        }
    });
    objects.push(Object {
        shape: Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
        },
        material: Material {
            mode: Mode::Diffuse,
            albedo: Color::new(0.4, 0.2, 0.1),
        }
    });
    objects.push(Object {
        shape: Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
        },
        material: Material {
            mode: Mode::Metallic(0.0),
            albedo: Color::new(0.7, 0.6, 0.5),
        }
    });
    SceneDef {
        camera: CameraDef {
            eye: Vec3::new(13.0, 2.0, 3.0),
            target: Vec3::ZERO,
            fov: 20.0,
            aperture: 0.2,
        },
        background: Background {
            top: Color::new(0.5, 0.7, 1.0),
            bottom: Color::WHITE,
        },
        objects,
    }
}
