use vec3::Vec3;
use ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    /// The distance along the ray at which the intersection occured
    pub t: f64,
    /// The intersecting ray
    pub ray: Ray,
    /// The point of intersection
    pub point: Vec3,
    /// A normal off the intersection
    pub normal: Vec3,
}

impl Intersection {
    /// Create a new Intersection instance
    ///
    /// # Arguments
    /// * `t`      - the distance along the ray at which the intersection occured
    /// * `ray`    - the intersecting ray
    /// * `normal` - a normal off the intersection
    ///
    pub fn new(t: f64, ray: Ray, normal: Vec3) -> Self {
        Intersection {
            t: t,
            ray: ray,
            point: ray.at(t),
            normal: normal.unit(),
        }
    }

    /// Returns the vector reflected by the ray off the normal
    pub fn reflected(&self) -> Vec3 {
        Vec3::reflect(self.ray.dir, self.normal)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Geo {
    /// A geometry representing a sphere
    Sphere { center: Vec3, radius: f64 },
}

impl Geo {
    /// Construct a new sphere
    pub fn sphere(center: Vec3, radius: f64) -> Self {
        Geo::Sphere { center, radius }
    }
}

/// Given a ray, a piece of geometry, and bounds on the ray determines if the
/// ray and geometry intersect. Returns an Option of Intersection which will be
/// Some if the intersection occured, and None if there was no intersection.
///
/// # Arguments
/// * `ray` - the ray
/// * `geo` - the geometry
///`* `min` - the minimum distance along the ray
/// * `max` - the maximum distance along the ray
///
pub fn intersects(ray: Ray, geo: &Geo, min: f64, max: f64) -> Option<Intersection> {
    match geo {
        // Handle intersection for a sphere
        &Geo::Sphere { center, radius } => {
            let intersect = |t| Some(Intersection::new(t, ray, (ray.at(t) - center) / radius));
            let oc = ray.origin - center;
            let a = Vec3::dot(ray.dir, ray.dir);
            let b = Vec3::dot(oc, ray.dir);
            let c = Vec3::dot(oc, oc) - radius * radius;
            let d = b * b - a * c;
            if d > 0.0 {
                let t = (-b - d.sqrt()) / a;
                if min < t && t < max {
                    return intersect(t);
                }
                let t = (-b + d.sqrt()) / a;
                if min < t && t < max {
                    return intersect(t);
                }
            }
            None
        }
    }
}
