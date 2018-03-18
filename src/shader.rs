use vec3::Vec3;
use scene::Scene;
use ray::Ray;
use std;

const EPSILON: f64 = 0.0001;
const MAX_DEPTH: u32 = 50;

/// Returns a color vector for a given subpixel into a scene
///
/// # Arguments
/// * `scene`  - the scene being rendered
/// * `x`      - the subpixel x position
/// * `y`      - the subpixel y position
/// * `width`  - the screen's width in pixels
/// * `height` - the screen's height in pixels
///
pub fn shader(scene: &Scene, x: f64, y: f64, width: f64, height: f64) -> Vec3 {
    let u = x / width;
    let v = y / height;
    let ray = scene.camera.ray(u, v);
    shade(scene, ray, 0)
}

/// The shader implementation of this ray tracer. Takes a scene and a ray and
/// and determiens the resulting color.
///
/// # Arguments
/// * `scene` - the scene being rendered
/// * `ray`   - the incoming ray
/// * `depth` - the number of recursive calls, cuts off at MAX_DEPTH
///
fn shade(scene: &Scene, ray: Ray, depth: u32) -> Vec3 {
    match scene.intersects(ray, EPSILON, std::f64::MAX) {
        Some((attenuation, scattered)) => {
            if depth < MAX_DEPTH {
                if let Some(scatter_ray) = scattered {
                    return attenuation * shade(scene, scatter_ray, depth + 1);
                }
            }
            Vec3::zeros()
        }
        None => background(ray),
    }
}

/// Returns a background color given a ray
fn background(ray: Ray) -> Vec3 {
    let t = (ray.dir.y + 1.0) * 0.5;
    (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
}
