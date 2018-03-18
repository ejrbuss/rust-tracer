use vec3::Vec3;
use ray::Ray;
use geo::Intersection;
use util::{rand, schlick};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    /// Material for lambertian (Diffuse) objects
    Lambertian { albedo: Vec3 },
    /// Material for metal (reflective) objects
    Metal { albedo: Vec3, fuzz: f64 },
    /// Material for dielectric (refractive) objects
    Dielectric { ref_idx: f64 },
}

impl Material {
    /// Creates a new lambertian material
    pub fn lambertian(albedo: Vec3) -> Self {
        Material::Lambertian { albedo }
    }
    /// Creates a new metal material
    pub fn metal(albedo: Vec3, fuzz: f64) -> Self {
        Material::Metal { albedo, fuzz }
    }
    /// Creates a new dielectric material
    pub fn dielectric(ref_idx: f64) -> Self {
        Material::Dielectric { ref_idx }
    }
}

pub fn shade(i: Intersection, mat: &Material) -> (Vec3, Option<Ray>) {
    match mat {
        // Handle material for lambertian
        &Material::Lambertian { albedo } => {
            let target = i.point + i.normal + Vec3::rand();
            (albedo, Some(Ray::new(i.point, target - i.point)))
        }
        // Handle material for metal
        &Material::Metal { albedo, fuzz } => {
            let scattered = Ray::new(i.point, i.reflected() + fuzz * Vec3::rand());
            if Vec3::dot(scattered.dir, i.normal) > 0.0 {
                (albedo, Some(scattered))
            } else {
                (albedo, None)
            }
        }
        // Handle material for dielectrics
        &Material::Dielectric { ref_idx } => {
            let (normal, k, cosine) = if Vec3::dot(i.ray.dir, i.normal) > 0.0 {
                (-i.normal, ref_idx, ref_idx * Vec3::dot(i.ray.dir, i.normal))
            } else {
                (i.normal, 1.0 / ref_idx, -Vec3::dot(i.ray.dir, i.normal))
            };
            let (refracted, p) = match Vec3::refract(i.ray.dir, normal, k) {
                Some(refracted) => (refracted, schlick(cosine, ref_idx)),
                None => (Vec3::ones(), 1.0),
            };
            (
                Vec3::ones(),
                Some(Ray::new(
                    i.point,
                    if rand() < p { i.reflected() } else { refracted },
                )),
            )
        }
    }
}
