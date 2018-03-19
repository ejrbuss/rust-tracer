use vec3::Vec3;
use ray::Ray;
use geo::Intersection;
use util::{rand, schlick};
use texture::Texture;

#[derive(Debug, Clone)]
pub enum Material {
    /// Material for lambertian (Diffuse) objects
    Lambertian { albedo: Texture },
    /// Material for metal (reflective) objects
    Metal { albedo: Texture, fuzz: f64 },
    /// Material for dielectric (refractive) objects
    Dielectric { albedo: Texture, ref_idx: f64 },
}

impl Material {
    /// Creates a new lambertian material
    pub fn lambertian(albedo: Texture) -> Self {
        Material::Lambertian { albedo }
    }
    /// Creates a new metal material
    pub fn metal(albedo: Texture, fuzz: f64) -> Self {
        Material::Metal { albedo, fuzz }
    }
    /// Creates a new dielectric material
    pub fn dielectric(albedo: Texture, ref_idx: f64) -> Self {
        Material::Dielectric { albedo, ref_idx }
    }

    /// Determine the shade at a given intersection
    pub fn shade(&self, i: &Intersection) -> (Vec3, Option<Ray>) {
        match self {
            // Handle material for lambertian
            &Material::Lambertian { ref albedo } => {
                let target = i.point + i.normal + Vec3::rand();
                (albedo.value(i), Some(Ray::from_intersection(i, target - i.point)))
            }
            // Handle material for metal
            &Material::Metal { ref albedo, fuzz } => {
                let scattered = Ray::from_intersection(i, i.reflected() + fuzz * Vec3::rand());
                (albedo.value(i), if Vec3::dot(scattered.dir, i.normal) > 0.0 {
                    Some(scattered)
                } else {
                    (None)
                })
            },
            // Handle material for dielectrics
            &Material::Dielectric { ref albedo, ref_idx } => {
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
                    albedo.value(i),
                    Some(Ray::from_intersection(i, if rand() < p {
                            i.reflected()
                        } else {
                            refracted
                        },
                    )),
                )
            }
        }
    }
}


