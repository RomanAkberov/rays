use rays::*;

fn main() -> RayResult<()> {
    let config = Config {
        image: ImageConfig {
            path: "ray-marcher.png".to_string(),
            width: 640,
            height: 480,
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
        shape: Box::new(Sphere {
            center: Vec3([0.0, -1000.0, 0.0]),
            radius: 1000.0,
        }),
        material: ground_material,
    });
    let mut random = Random::new(42);
    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_sphere = random.probability(0.5);
            let choose_mat = random.in_01();
            let center = Vec3([
                a as Float + 0.9 * random.in_01(), 
                0.2,
                b as Float + 0.9 * random.in_01(),
            ]);
            if center.distance(Vec3([4.0, 0.2, 0.0])) > 0.9 {
                let material = if choose_mat < 0.8 {
                    // diffuse
                    Material {
                        mode: Mode::Diffuse,
                        albedo: Color {
                            r: random.in_01() * random.in_01(),
                            g: random.in_01() * random.in_01(),
                            b: random.in_01() * random.in_01(),
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
                let shape = if choose_sphere {
                    Box::new(Sphere {
                        center,
                        radius: 0.2,
                    }) as Box<dyn Shape>
                } else {
                    Box::new(Aabb3 {
                        min: center - Vec3::splat(0.2),
                        max: center + Vec3::splat(0.2),
                    })
                };
                objects.push(Object {
                    shape,
                    material,
                });
            }
        }
    }
    objects.push(Object {
        shape: Box::new(Sphere {
            center: Vec3([0.0, 1.0, 0.0]),
            radius: 1.0,
        }),
        material: Material {
            mode: Mode::Dielectric(1.5),
            albedo: Color::WHITE,
        }
    });
    objects.push(Object {
        shape: Box::new(Sphere {
            center: Vec3([-4.0, 1.0, 0.0]),
            radius: 1.0,
        }),
        material: Material {
            mode: Mode::Diffuse,
            albedo: Color::new(0.4, 0.2, 0.1),
        }
    });
    objects.push(Object {
        shape: Box::new(Sphere {
            center: Vec3([4.0, 1.0, 0.0]),
            radius: 1.0,
        }),
        material: Material {
            mode: Mode::Metallic(0.0),
            albedo: Color::new(0.7, 0.6, 0.5),
        }
    });
    SceneDef {
        camera: CameraDef {
            eye: Vec3([13.0, 2.0, 3.0]),
            target: Vec3::ZERO,
            fov: 20.0,
            aperture: 0.0,
        },
        background: Background {
            top: Color::new(0.5, 0.7, 1.0),
            bottom: Color::WHITE,
        },
        objects,
    }
}
