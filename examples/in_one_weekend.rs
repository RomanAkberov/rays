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
        renderer: RenderMode::RayMarcher,
        show_progress: true,
    };
    run_scene(&config, scene())
}

fn scene() -> SceneDef {
    let mut objects = Vec::new();
    let ground_material = Material::Diffuse(
        Texture::Checker(
            Box::new(Texture::Constant(Color::new(0.2, 0.3, 0.1))), 
            Box::new(Texture::Constant(Color::new(0.9, 0.9, 0.9))),
        ),
    );
    objects.push(Object {
        shape: Shape::Sphere(Sphere {
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
                    Material::Diffuse(
                        Texture::Constant(Color::new(
                            random.in_01() * random.in_01(),
                            random.in_01() * random.in_01(),
                            random.in_01() * random.in_01(),
                        ))
                    )
                } else if choose_mat < 0.95 {
                    // metal
                    Material::Metallic(Color::new(
                        random.in_range(0.5, 1.0),
                        random.in_range(0.5, 1.0),
                        random.in_range(0.5, 1.0),
                    ), random.in_range(0.0, 0.5))
                } else {
                    Material::Dielectric(1.5)
                };
                let shape = if choose_sphere {
                    Shape::Sphere(Sphere {
                        center,
                        radius: 0.2,
                    })
                } else {
                    Shape::Cuboid(Aabb3 {
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
        shape: Shape::Sphere(Sphere {
            center: Vec3([0.0, 1.0, 0.0]),
            radius: 1.0,
        }),
        material: Material::Dielectric(1.5),
    });
    objects.push(Object {
        shape: Shape::Sphere(Sphere {
            center: Vec3([-4.0, 1.0, 0.0]),
            radius: 1.0,
        }),
        material: Material::Diffuse(
            Texture::Constant(Color::new(0.4, 0.2, 0.1)),
        ),
    });
    objects.push(Object {
        shape: Shape::Sphere(Sphere {
            center: Vec3([4.0, 1.0, 0.0]),
            radius: 1.0,
        }),
        material: Material::Metallic(
            Color::new(0.7, 0.6, 0.5),
            0.0,
        ),
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
