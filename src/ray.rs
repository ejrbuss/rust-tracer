use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {

    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin: origin, dir: dir.unit() }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }

}