use vec3::Vec3;
use ray::Ray;
use std;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub llc: Vec3,
    pub hrz: Vec3,
    pub vrt: Vec3,
    pub lens: f64
}

impl Camera {

    pub fn new(
        from: Vec3,
        to: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus: f64
    ) -> Self {

        // Calculate screen width and height
        let height = ((vfov * std::f64::consts::PI / 180.0) / 2.0).tan() * 2.0;
        let width  = aspect * height;

        // Calculate orthanormal basis
        let w = (from - to).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u).unit();

        let hrz = width * u * focus;
        let vrt = height * v * focus;

        Camera {
            origin: from,
            llc:    from - hrz / 2.0 - vrt / 2.0 - w * focus,
            hrz:    hrz,
            vrt:    vrt,
            lens:   aperture / 2.0,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let rd  = self.lens * Vec3::rand_disc();
        let off = u * rd.x + v * rd.y;
        Ray::new(
            self.origin + off,
            self.llc + u * self.hrz + v * self.vrt - self.origin - off
        )
    }

}