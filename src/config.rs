extern crate toml;
extern crate image;

use scene::Scene;
use frame::Frame;
use image::{ PNG, JPEG, GIF, WEBP, BMP, ICO, };
use std::fmt::{ Display, Formatter };
use scenes::{
    empty,
    sphere,
    spheres,
    random_scene,
    two_spheres,
    two_perlin_spheres,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The output format, either PNG, JPEG, GIF, WEBP, BMP, or ICO
    format: String,
    /// The output path
    out: String,
    /// The scene to render
    scene: String,
    /// The number of samples to take
    samples: u32,
    /// The render width
    width: u32,
    /// The render height
    height: u32,
}

impl Config {
    // Retrieve a default configuration
    pub fn default() -> Self {
        Config {
            format:  String::from("PNG"),
            out:     String::from("./out.png"),
            scene:   String::from("random_scene"),
            samples: 1,
            width:   400,
            height:  200,
        }
    }

    /// The output format, either PNG, JPEG, GIF, WEBP, BMP, PPM, or ICO
    pub fn format(&self) -> image::ImageFormat {
        match self.format.to_uppercase().as_ref() {
            "PNG"  => PNG,
            "JPG"  => JPEG,
            "JPEG" => JPEG,
            "GIF"  => GIF,
            "WEBP" => WEBP,
            "BMP"  => BMP,
            "ICO"  => ICO,
            other  => panic!("Unrecognized format {}!", other),
        }
    }

    /// Get the scene to render
    pub fn scene(&self) -> Scene {
        match self.scene.to_lowercase().as_ref() {
            "empty"              => empty(),
            "sphere"             => sphere(),
            "spheres"            => spheres(),
            "random_scene"       => random_scene(self.aspect()),
            "two_spheres"        => two_spheres(self.aspect()),
            "two_perlin_spheres" => two_perlin_spheres(self.aspect()),
            other                => panic!("Unrecognized scene {}!", other),
        }
    }

    /// Get the aspect ratio
    pub fn aspect(&self) -> f64 {
        (self.width as f64) / (self.height as f64)
    }

    /// Get the frame
    pub fn frame(&self) -> Frame {
        Frame::new(self.width, self.height, self.samples)
    }

    /// Runs the given config
    pub fn run(&self) {
        let path: &str = self.out.as_ref();
        self.frame().render_to(&self.scene(), path, self.format());
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f,
"rust-tracer Configuration [\
\n    format:  {}\
\n    out:     {}\
\n    scene:   {}\
\n    samples: {}\
\n    width:   {}\
\n    height:  {}\
\n]",
            self.format,
            self.out,
            self.scene,
            self.samples,
            self.width,
            self.height,
        )
    }
}