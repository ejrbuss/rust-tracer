extern crate rand;
extern crate image;
extern crate rayon;

mod util;
mod color;
mod frame;
mod vec3;
mod ray;
mod shader;
mod camera;
mod geo;
mod scene;
mod material;

use util::rand;
use frame::Frame;
use vec3::Vec3;
use camera::Camera;
use scene::Scene;
use geo::Geo;
use material::Material;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;
const SAMPLES: u32 = 10;

const ASPECT: f64 = (WIDTH as f64) / (HEIGHT as f64);

fn main() {

    let scene = random_scene();
    let mut frame = Frame::new(WIDTH, HEIGHT, SAMPLES);
    frame.update(&scene);
    frame.save("out.png");
}

fn random_scene() -> Scene {
    let mut objects = Vec::new();
    objects.push((
        Geo::sphere(Vec3::new(0.0, -1000.0, 0.0), 1000.0),
        Material::lambertian(Vec3::new(0.5, 0.5, 0.5))
    ));
    for ia in -11..11 {
        for ib in -11..11 {
            let a = ia as f64;
            let b = ib as f64;
            let choose_mat = rand();
            let center = Vec3::new(a + 0.9 * rand(), 0.2, b + 0.9 * rand());
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    objects.push((
                        Geo::sphere(center, 0.2),
                        Material::lambertian(Vec3::new(rand() * rand(), rand() * rand(), rand() * rand()))
                    ));
                } else if choose_mat < 0.95 {
                    objects.push((
                        Geo::sphere(center, 0.2),
                        Material::metal(Vec3::new(0.5 * (1.0 + rand()), 0.5 * (1.0 + rand()), 0.5 * (1.0 + rand())), 0.5 * rand())
                    ));
                } else {
                    objects.push((
                        Geo::sphere(center, 0.2),
                        Material::dielectric(1.5)
                    ))
                }
            }
        }
    }
    objects.push((
        Geo::sphere(Vec3::new(0.0, 1.0, 0.0), 1.0),
        Material::dielectric(1.5)
    ));
    objects.push((
        Geo::sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0),
        Material::lambertian(Vec3::new(0.4, 0.2, 0.1))
    ));
    objects.push((
        Geo::sphere(Vec3::new(4.0, 1.0, 0.0), 1.0),
        Material::metal(Vec3::new(0.7, 0.6, 0.5), 0.0)
    ));

    let from  = Vec3::new(13.0, 2.0, 3.0);
    let at    = Vec3::new(0.0, 0.0, 0.0);
    let vup   = Vec3::new(0.0, 1.0, 0.0);
    Scene::new(
        Camera::new(from, at, vup, 20.0, ASPECT, 0.2, (from - at).len()),
        objects
    )
}