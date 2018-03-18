use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// The originating point of the ray
    pub origin: Vec3,
    /// The direction of the ray
    pub dir: Vec3,
}

impl Ray {
    /// Creates a new Ray.
    ///
    /// # Arguments
    /// * `origin` - the originating point of the ray
    /// * `dir`    - the direction of the ray
    ///
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin: origin, dir: dir.unit(), }
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
