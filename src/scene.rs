use camera::Camera;
use vec3::Vec3;
use ray::Ray;
use geo::Geo;

#[derive(Debug)]
pub struct Scene {
    /// The scene's camera
    pub camera: Camera,
    /// The secne's
    pub geometry: Geo,
}

impl Scene {
    /// Creates a new scene.
    pub fn new(camera: Camera, geometry: Geo) -> Self {
        Scene { camera, geometry }
    }

    /// Checks if a ray intersects with the scene's geometry
    pub fn intersects(&self, ray: Ray, min: f64, max: f64) -> Option<(Vec3, Option<Ray>)> {
        if let Some(i) = self.geometry.intersects(ray, min, max) {
            Some(i.material.shade(&i))
        } else {
            None
        }
    }
}
