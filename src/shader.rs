use color::Color;
use vec3::Vec3;
use scene::Scene;
use ray::Ray;
use std;

const GAMMA: f64 = 0.5;
const EPSILON: f64 = 0.0001;
const MAX_DEPTH: u32 = 50;

pub fn shader(scene: &Scene, x: f64, y: f64, width: f64, height: f64) -> Color {
    let u   = x / width;
    let v   = y / height;
    let ray = scene.camera.ray(u, v);
    shade(scene, ray, 0).color(GAMMA)
}

fn shade(scene: &Scene, ray: Ray, depth: u32) -> Vec3 {
    match scene.intersects(ray, EPSILON, std::f64::MAX) {
        Some((attenuation, scattered)) => {
            if depth < MAX_DEPTH {
                if let Some(scatter_ray) = scattered {
                    return attenuation * shade(scene, scatter_ray, depth + 1);
                }
            }
            Vec3::zeros()
        },
        None => {
            let t = (ray.dir.y + 1.0) * 0.5;
            (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
        },
    }
}