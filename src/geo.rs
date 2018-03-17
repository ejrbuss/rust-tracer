use vec3::Vec3;
use ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub ray: Ray,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Intersection {

    pub fn new(t: f64, ray: Ray, normal: Vec3) -> Self {
        Intersection {
            t:      t,
            ray:    ray,
            point:  ray.at(t),
            normal: normal.unit(),
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub enum Geo {
    Sphere {
        center: Vec3,
        radius: f64,
    },
}

impl Geo {

    pub fn sphere(center: Vec3, radius: f64) -> Self {
        Geo::Sphere { center, radius }
    }

}

pub fn intersects(ray: Ray, geo: &Geo, min: f64, max: f64) -> Option<Intersection> {
    match geo {
        &Geo::Sphere { center, radius } => {

            let intersect = |t| Some(Intersection::new(
                t, ray, (ray.at(t) - center) / radius
            ));
            let oc = ray.origin - center;
            let a = Vec3::dot(ray.dir, ray.dir);
            let b = Vec3::dot(oc, ray.dir);
            let c = Vec3::dot(oc, oc) - radius * radius;
            let d = b * b - a * c;

            if d > 0.0 {
                let t = (-b - d.sqrt()) / a;
                if min < t && t < max { return intersect(t); }
                let t = (-b + d.sqrt()) / a;
                if min < t && t < max { return intersect(t); }
            }
            None
        },
    }
}