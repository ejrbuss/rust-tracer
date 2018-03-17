use vec3::Vec3;
use ray::Ray;
use geo::Intersection;
use util::{ rand, schlick };

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian {
        albedo: Vec3,
    },
    Metal {
        albedo: Vec3,
        fuzz: f64,
    },
    Dielectric {
        ref_idx: f64,
    }
}

impl Material {

    pub fn lambertian(albedo: Vec3) -> Self {
        Material::Lambertian { albedo }
    }

    pub fn metal(albedo: Vec3, fuzz: f64) -> Self {
        Material::Metal { albedo, fuzz }
    }

    pub fn dielectric(ref_idx: f64) -> Self {
        Material::Dielectric { ref_idx }
    }

}

pub fn shade(i: Intersection, mat: &Material) -> (Vec3, Option<Ray>) {
    match mat {
        &Material::Lambertian { albedo } => {
            let target = i.point + i.normal + Vec3::rand();
            (albedo, Some(Ray::new(i.point, target - i.point)))
        },
        &Material::Metal { albedo, fuzz } => {
            let reflected = Vec3::reflect(i.ray.dir, i.normal);
            let scattered = Ray::new(i.point, reflected + fuzz * Vec3::rand());
            if Vec3::dot(scattered.dir, i.normal) > 0.0 {
                (albedo, Some(scattered))
            } else {
                (albedo, None)
            }
        },
        &Material::Dielectric { ref_idx } => {
            let reflected = Vec3::reflect(i.ray.dir, i.normal);
            let (normal, k, cosine) = if Vec3::dot(i.ray.dir, i.normal) > 0.0 {
                (-i.normal, ref_idx, ref_idx * Vec3::dot(i.ray.dir, i.normal))
            } else {
                (i.normal, 1.0 / ref_idx, -Vec3::dot(i.ray.dir, i.normal))
            };
            let (refracted, p) = match Vec3::refract(i.ray.dir, normal, k) {
                Some(refracted) => (refracted, schlick(cosine, ref_idx)),
                None => (Vec3::ones(), 1.0),
            };
            (Vec3::ones(), Some(Ray::new(i.point, if rand() < p { reflected } else { refracted })))
        },
    }
}