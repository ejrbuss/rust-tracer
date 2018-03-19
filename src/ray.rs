use vec3::Vec3;
use geo::Intersection;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// The originating point of the ray
    pub origin: Vec3,
    /// The direction of the ray
    pub dir: Vec3,
    /// The time of the ray
    pub time: f64,
}

impl Ray {
    /// Creates a new Ray.
    ///
    /// # Arguments
    /// * `origin` - the originating point of the ray
    /// * `dir`    - the direction of the ray
    ///
    pub fn new(origin: Vec3, dir: Vec3, time: f64) -> Self {
        Ray { origin, dir: dir.unit(), time, }
    }

    /// Creates a new Ray starting from an intersection going off in the given
    /// direction at the same time as the incident ray.
    ///
    /// # Arguments
    /// * `i`   - the intersection
    /// * `dir` - the outgoing direction of the ray
    ///
    pub fn from_intersection(i: &Intersection, dir: Vec3) -> Self {
        Ray::new(i.point, dir, i.ray.time)
    }

    /// Returns the point along the ray distance t from the origin
    ///
    /// # Arguments
    /// * `t` the distance along the ray
    ///
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}
