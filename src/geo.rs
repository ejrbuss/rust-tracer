use util::rand;
use vec3::Vec3;
use ray::Ray;
use aabb::AABB;
use material::Material;
use std::cmp::Ordering::Equal;

#[derive(Debug, Clone)]
pub struct Intersection {
    /// The distance along the ray at which the intersection occured
    pub t: f64,
    /// The intersecting ray
    pub ray: Ray,
    /// The point of intersection
    pub point: Vec3,
    /// A normal off the intersection
    pub normal: Vec3,
    /// The material of the surface
    pub material: Material,
}

impl Intersection {
    /// Create a new Intersection instance
    ///
    /// # Arguments
    /// * `t`      - the distance along the ray at which the intersection occured
    /// * `ray`    - the intersecting ray
    /// * `normal` - a normal off the intersection
    ///
    pub fn new(t: f64, ray: Ray, normal: Vec3, mat: Material) -> Self {
        Intersection {
            t:        t,
            ray:      ray,
            point:    ray.at(t),
            normal:   normal.unit(),
            material: mat,
        }
    }

    /// Returns the vector reflected by the ray off the normal
    pub fn reflected(&self) -> Vec3 {
        Vec3::reflect(self.ray.dir, self.normal)
    }
}

#[derive(Debug, Clone)]
pub enum Geo {
    /// A geometry representing a set of geometry
    List {
        children: Vec<Geo>,
    },
    /// A geometry representing a BVH
    #[allow(non_camel_case_types)]
    BVH_node {
        bbox: AABB,
        left: Box<Geo>,
        right: Box<Geo>,
    },
    /// A geometry representing a sphere
    Sphere {
        center0: Vec3,
        center1: Vec3,
        radius: f64,
        time0: f64,
        time1: f64,
        material: Material,
    },
}

impl Geo {
    /// Construct a new geometry list
    pub fn list(children: Vec<Geo>) -> Self {
        Geo::List {
            children: children,
        }
    }

    /// Construct a new BVH node
    pub fn bvh_node(mut children: Vec<Geo>) -> Self {
        // Comparison function generator
        let compare = |i: u32| {
            move |a: &Geo, b: &Geo| {
                match (a.bounding_box(), b.bounding_box()) {
                    (
                        AABB::BBox { min: min1, .. },
                        AABB::BBox { min: min2, .. },
                    ) => min1[i].partial_cmp(&min2[i]).unwrap_or(Equal),
                    _ => Equal,
                }
            }
        };
        // Sort children
        let axis = (3.0 * rand()) as u32;
        children.sort_by(compare(axis));
        // Determine left and right nodes
        let n = children.len();
        let (left, right) = match n {
            // Special cases, return leaf
            0 => return Geo::list(Vec::new()),
            1 => return children.remove(0),
            // Left and right are leaves
            2 => (children.remove(0), children.remove(0)),
            // Child nodes
            _ =>{
                let mut chunks = children.chunks_mut(n / 2 + 1);
                let left  = Geo::bvh_node(chunks.next().unwrap().to_vec());
                let right = Geo::bvh_node(chunks.next().unwrap().to_vec());
                (left, right)
            },
        };
        Geo::BVH_node {
            bbox: AABB::bound_boxes(
                left.bounding_box(),
                right.bounding_box(),
            ),
            left:  Box::new(left),
            right: Box::new(right),
        }
    }

    /// Construct a new sphere
    pub fn sphere(center: Vec3, radius: f64, mat: Material) -> Self {
        Geo::Sphere {
            center0:  center,
            center1:  center,
            radius:   radius,
            time0:    0.0,
            time1:    1.0,
            material: mat,
        }
    }

    /// Construct a new moving sphere
    pub fn moving_sphere(c0: Vec3, c1: Vec3, r: f64, t0: f64, t1: f64, mat: Material) -> Self {
        Geo::Sphere {
            center0:  c0,
            center1:  c1,
            radius:   r,
            time0:    t0,
            time1:    t1,
            material: mat,
        }
    }

    /// Find the center of a sphere at a given time
    pub fn center(&self, time: f64) -> Vec3 {
        match self {
            &Geo::Sphere { center0, center1, time0, time1, .. } => center0
                + ((time - time0) / (time1 - time0))
                * (center1 - center0),
            other => panic!("{:?} does not havea center!", other),
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
    pub fn intersects(&self, ray: Ray, min: f64, max: f64) -> Option<Intersection> {
        match self {
            // Handle intersection for a list
            &Geo::List { ref children } => {
                let mut closest = None;
                let mut cmax    = max;
                for geo in children {
                    if let Some(i) = geo.intersects(ray, min, cmax) {
                        cmax    = i.t;
                        closest = Some(i);
                    }
                }
                closest
            },
            // Handle intersection for a BVH
            &Geo::BVH_node { ref bbox, ref left, ref right, .. } => {
                if bbox.hit(ray, min, max) {
                    match (
                        left.intersects(ray, min, max),
                        right.intersects(ray, min, max),
                    ) {
                        (Some(ileft), Some(iright)) => {
                            if ileft.t < iright.t {
                                Some(ileft)
                            } else {
                                Some(iright)
                            }
                        },
                        // Pass through the results
                        (Some(ileft), _)  => Some(ileft),
                        (_, Some(iright)) => Some(iright),
                        _                 => None,
                    }
                } else {
                    None
                }
            },
            // Handle intersection for a sphere
            &Geo::Sphere { radius, ref material, .. } => {
                let center = self.center(ray.time);
                let intersect = |t| Some(Intersection::new(
                    t, ray,
                    (ray.at(t) - center) / radius,
                    material.clone()
                ));
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

    /// Return the bounding box of a piece of geometry
    pub fn bounding_box(&self) -> AABB {
        match self {
            // Determine the bounding box for a list
            &Geo::List { ref children } => {
                let mut bbox = AABB::None;
                for geo in children {
                    bbox = AABB::bound_boxes(bbox, geo.bounding_box());
                }
                bbox
            },
            // Determine the bounding box for a BVH (trivial)
            &Geo::BVH_node { bbox, .. } => { bbox },
            // Determine the bounding box for a sphere
            &Geo::Sphere { center0, center1, radius, .. } => {
                let vradius = Vec3::ones() * radius;
                let box1    = AABB::new(center0 - vradius, center0 + vradius);
                let box2    = AABB::new(center1 - vradius, center1 + vradius);
                AABB::bound_boxes(box1, box2)
            },

        }
    }
}
