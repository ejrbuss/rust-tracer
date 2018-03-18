use camera::Camera;
use vec3::Vec3;
use ray::Ray;
use geo::{intersects, Geo};
use material::{shade, Material};

#[derive(Debug)]
pub struct Scene {
    /// The scene's camera
    pub camera: Camera,
    /// The secne's
    pub geometry: Vec<(Geo, Material)>,
}

impl Scene {
    /// Creates a new scene.
    pub fn new(camera: Camera, geometry: Vec<(Geo, Material)>) -> Self {
        Scene { camera, geometry }
    }

    pub fn intersects(&self, ray: Ray, min: f64, max: f64) -> Option<(Vec3, Option<Ray>)> {
        let mut closest = None;
        let mut cmax = max;
        for &(geo, mat) in &self.geometry {
            if let Some(i) = intersects(ray, &geo, min, cmax) {
                closest = Some(shade(i, &mat));
                cmax = i.t;
            }
        }
        closest
    }
}
