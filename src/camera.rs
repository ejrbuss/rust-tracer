use util::rand;
use vec3::Vec3;
use ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// The origin of the camera
    pub origin: Vec3,
    /// The lower left hand corner of screenspace
    pub llc: Vec3,
    /// The horizontal distance across screenspace
    pub hrz: Vec3,
    /// The vertical distance across screenspace
    pub vrt: Vec3,
    /// The lens length
    pub lens: f64,
    /// Shutter opening time
    pub time_open: f64,
    /// Shutter closing time
    pub time_close: f64,
    /// The time difference
    pub time_difference: f64,
}

impl Camera {
    /// Constructs a new camera.
    ///
    /// # Arguments
    /// * `from`     - where the camera is looking from
    /// * `to`       - what the camera is looking at
    /// * `vup`      - a vertical up vector
    /// * `vfov`     - the vertical field of view in degrees
    /// * `aspect`   - the screensapce aspect ratio
    /// * `aperture` - the camera's aperture size
    /// * `focus`    - the focus distance, by default the distance from `from`
    ///                to `at`
    ///
    pub fn new(
        from: Vec3,
        to: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        defocus: Option<(f64, f64)>,
        time: Option<(f64, f64)>
    ) -> Self {
        let height = (vfov.to_radians() / 2.0).tan() * 2.0;
        let width  = aspect * height;

        // Calcluate basis from vup and the look direction
        let w = (from - to).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u).unit();

        // If defocus is None, default to identity values
        let (aperture, focus) = defocus.unwrap_or((0.0, 1.0));
        let hrz = width * u * focus;
        let vrt = height * v * focus;

        // If time is None, default is no time passing
        let (time_open, time_close) = time.unwrap_or((0.0, 0.0));

        Camera {
            origin:          from,
            llc:             from - hrz / 2.0 - vrt / 2.0 - w * focus,
            hrz:             hrz,
            vrt:             vrt,
            lens:            aperture / 2.0,
            time_open:       time_open,
            time_close:      time_close,
            time_difference: time_close - time_open,
        }
    }

    /// Creates a default camera
    pub fn default() -> Self {
        Camera {
            origin:          Vec3::zeros(),
            llc:             Vec3::new(-2.0, -1.0, -1.0),
            hrz:             Vec3::new(4.0, 0.0, 0.0),
            vrt:             Vec3::new(0.0, 2.0, 0.0),
            lens:            0.0,
            time_open:       0.0,
            time_close:      0.0,
            time_difference: 0.0
        }
    }

    /// Returns an array given a screenspace coordinate (u, v)
    ///
    /// # Arguments
    /// * `u` - x screenspace coordinate
    /// * `v` - y screenspace coordinate
    ///
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let Vec3 { x, y, .. } = self.lens * Vec3::rand_disc();
        let off    = u * x + v * y;
        let origin = self.origin - off;
        let dir    = self.llc + u * self.hrz + v * self.vrt - self.origin + off;
        let time   = self.time_open + rand() * self.time_difference;
        Ray::new(origin, dir, time)
    }
}
