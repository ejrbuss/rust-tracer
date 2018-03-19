use vec3::Vec3;
use ray::Ray;

#[derive(Debug, Clone, Copy)]
pub enum AABB {
    /// A bounding box that exists
    BBox {
        min: Vec3,
        max: Vec3,
    },
    /// A bounding box that does not exist
    None,
}

impl AABB {
    /// Creates a new bounding box
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB::BBox { min, max }
    }

    /// Find the bounding box around two boxes
    pub fn bound_boxes(box1: AABB, box2: AABB) -> AABB{
        match (box1, box2) {
            (
                AABB::BBox { min: min1, max: max1 },
                AABB::BBox { min: min2, max: max2 },
            ) => AABB::new(
                Vec3::new(
                    fmin(min1.x, min2.x),
                    fmin(min1.y, min2.y),
                    fmin(min1.z, min2.z)
                ),
                Vec3::new(
                    fmax(max1.x, max2.x),
                    fmax(max1.y, max2.y),
                    fmax(max1.z, max2.z)
                )
            ),
            (AABB::BBox { .. }, _) => box1,
            _                      => box2,
        }
    }

    /// Returns true if the ray hits the bounding box
    pub fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> bool {
        match self {
            &AABB::BBox { min, max } => {
                let mut tmin = tmin;
                let mut tmax = tmax;
                for i in 0..3 {
                    let inv_d = 1.0 / ray.dir[i];
                    let a = (min[i] - ray.origin[i]) * inv_d;
                    let b = (max[i] - ray.origin[i]) * inv_d;
                    let (t0, t1) = (fmin(a, b), fmax(a, b));
                    tmin = fmax(t0, tmin);
                    tmax = fmin(t1, tmax);
                    if tmax <= tmin { return false; }
                }
                true
            },
            &AABB::None => false,
        }

    }
}

fn fmin(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn fmax(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
