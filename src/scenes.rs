use util::rand;
use vec3::Vec3;
use scene::Scene;
use camera::Camera;
use geo::Geo;
use material::Material;
use texture::Texture;

pub fn empty() -> Scene {
    Scene::new(Camera::default(), Geo::list(Vec::new()))
}

pub fn sphere() -> Scene {
    Scene::new(
        Camera::default(),
        Geo::sphere(
            Vec3::new(0.0, 0.0, -1.0), 0.5,
            Material::lambertian(Texture::solid(Vec3::new(0.8, 0.3, 0.5)))
        )
    )
}

pub fn spheres() -> Scene {
    Scene::new(
        Camera::default(),
        Geo::list(vec![
            Geo::sphere(
                Vec3::new(0.0, 0.0, -1.0), 0.5,
                Material::lambertian(Texture::solid(Vec3::new(0.8, 0.3, 0.5)))
            ),
            Geo::sphere(
                Vec3::new(0.0, -100.5, -1.0), 100.0,
                Material::lambertian(Texture::solid(Vec3::new(0.8, 0.8, 0.0)))
            ),
            Geo::sphere(
                Vec3::new(1.0, 0.0, -1.0), 0.5,
                Material::metal(Texture::solid(Vec3::new(0.8, 0.6, 0.2)), 0.3)
            ),
            Geo::sphere(
                Vec3::new(-1.0, 0.0, -1.0), 0.5,
                Material::dielectric(Texture::solid(Vec3::ones()), 1.5)
            ),
            Geo::sphere(
                Vec3::new(-1.0, 0.0, -1.0), -0.45,
                Material::dielectric(Texture::solid(Vec3::ones()), 1.5)
            )
        ]
    ))
}

pub fn random_scene(aspect: f64) -> Scene {
    let mut objects = Vec::new();
    objects.push(Geo::sphere(
        Vec3::new(0.0, -1000.0, 0.0), 1000.0,
        Material::lambertian(Texture::checker(
            Texture::solid(Vec3::new(0.2, 0.3, 0.1)),
            Texture::solid(Vec3::new(0.9, 0.9, 0.9)),
            1.0
        ))
    ));
    for ia in -11..11 {
        for ib in -11..11 {
            let a = ia as f64;
            let b = ib as f64;
            let choose_mat = rand();
            let center = Vec3::new(a + 0.9 * rand(), 0.2, b + 0.9 * rand());
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    objects.push(Geo::moving_sphere(
                        center,
                        center + Vec3::new(0.0, 0.5, 0.0) * rand(),
                        0.2,
                        0.0,
                        1.0,
                        Material::lambertian(Texture::solid(Vec3::new(
                            rand() * rand(),
                            rand() * rand(),
                            rand() * rand(),
                        ))),
                    ));
                } else if choose_mat < 0.95 {
                    objects.push(Geo::sphere(
                        center, 0.2,
                        Material::metal(
                            Texture::solid(Vec3::new(
                                0.5 * (1.0 + rand()),
                                0.5 * (1.0 + rand()),
                                0.5 * (1.0 + rand()),
                            )),
                            0.5 * rand(),
                        ),
                    ));
                } else {
                    objects.push(Geo::sphere(
                        center, 0.2,
                        Material::dielectric(Texture::solid(Vec3::ones()), 1.5)))
                }
            }
        }
    }
    objects.push(Geo::sphere(
        Vec3::new(-4.0, 1.0, 0.0), 1.0,
        Material::lambertian(Texture::solid(Vec3::new(0.1, 0.6, 0.8)))
    ));
    objects.push(Geo::sphere(
        Vec3::new(0.0, 1.0, 0.0), 1.0,
        Material::metal(Texture::solid(Vec3::new(0.8, 0.6, 0.3)), 0.3)
    ));
    objects.push(Geo::list(vec![
        Geo::sphere(
            Vec3::new(4.0, 1.0, 0.0), 1.0,
            Material::dielectric(Texture::solid(Vec3::new(1.0, 0.9, 0.9)), 1.5)
        ),
        Geo::sphere(
            Vec3::new(4.0, 1.0, 0.0), -0.95,
            Material::dielectric(Texture::solid(Vec3::new(1.0, 0.9, 0.9)), 1.5)
        ),
    ]));

    let from    = Vec3::new(13.0, 2.0, 3.0);
    let at      = Vec3::new(0.0, 0.0, 0.0);
    let vup     = Vec3::new(0.0, 1.0, 0.0);
    let vfov    = 20.0;
    let defocus = Some((0.0, 10.0));
    let time    = Some((0.0, 1.0));
    Scene::new(
        Camera::new(from, at, vup, vfov, aspect, defocus, time),
        //Geo::list(objects),
        Geo::bvh_node(objects),
    )
}

pub fn two_spheres(aspect: f64) -> Scene {
    let from    = Vec3::new(13.0, 2.0, 3.0);
    let at      = Vec3::zeros();
    let vup     = Vec3::new(0.0, 1.0, 0.0);
    let vfov    = 20.0;
    let defocus = Some((0.2, 10.0));
    let checker = Texture::checker(
        Texture::solid(Vec3::new(0.0, 0.0, 0.0)),
        Texture::solid(Vec3::new(0.8, 0.5, 0.0)),
        10.0
    );
    Scene::new(
        Camera::new(from, at, vup, vfov, aspect, defocus, None),
        Geo::bvh_node(vec![
            Geo::sphere(
                Vec3::new(0.0, -10.0, 0.0), 10.0,
                Material::lambertian(checker.clone()),
            ),
            Geo::sphere(
                Vec3::new(0.0, 10.0, 0.0), 10.0,
                Material::lambertian(checker.clone()),
            )
        ])
    )
}

pub fn two_perlin_spheres(aspect: f64) -> Scene {
    let from    = Vec3::new(13.0, 2.0, 3.0);
    let at      = Vec3::zeros();
    let vup     = Vec3::new(0.0, 1.0, 0.0);
    let vfov    = 20.0;
    let defocus = Some((0.1, 10.0));
    Scene::new(
        Camera::new(from, at, vup, vfov, aspect, defocus, None),
        Geo::bvh_node(vec![
            Geo::sphere(
                Vec3::new(0.0, -1000.0, 0.0), 1000.0,
                Material::lambertian(Texture::noise()),
            ),
            Geo::sphere(
                Vec3::new(0.0, 2.0, 0.0), 2.0,
                Material::lambertian(Texture::noise()),
            )
        ])
    )
}