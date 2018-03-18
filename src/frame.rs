use rayon::prelude::*;
use std::path::Path;
use std::fs::File;
use vec3::Vec3;
use util::rand;
use shader::shader;
use scene::Scene;
use image::{ImageBuffer, ImageRgb8, Rgb, PNG};

const GAMMA: f64 = 0.5;

#[derive(Debug)]
pub struct Frame {
    /// The frame width in pixels
    pub width: u32,
    /// THe frame height in pixels
    pub height: u32,
    /// The number of samples to take per pixel
    pub samples: u32,
    /// A vector of all coordinates in the frame
    coords: Vec<(u32, u32)>,
    /// The frame buffer
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Frame {

    /// Create a new frame with a given width, height, and sample rate.
    ///
    /// # Arguments
    /// `width`   - the frame width in pixels
    /// `height`  - the frame height in pixels
    /// `samples` - the number of samples to take per pixel
    ///
    pub fn new(width: u32, height: u32, samples: u32) -> Self {
        // Generate coords at construction, so it only has to be done once
        let mut coords = Vec::new();
        for y in 0..height {
            for x in 0..width {
                // Adjust for ImageBuffer y coordinates being backwards
                coords.push((x, height - y - 1));
            }
        }
        Frame {
            width: width,
            height: height,
            samples: samples,
            coords: coords,
            buffer: ImageBuffer::new(width, height),
        }
    }

    /// Get the next frame given a scene.
    ///
    /// # Arguments
    /// `scene` the scene to render
    ///
    pub fn render(&mut self, scene: &Scene) {
        // In parallel shade the pixels
        let pixels: Vec<_> = self.coords
            .par_iter()
            .map(|&(x, y): &(u32, u32)| {
                let mut sum = Vec3::zeros();
                for _ in 0..self.samples {
                    sum = sum + shader(
                        scene,
                        (x as f64) + rand(),
                        (y as f64) + rand(),
                        self.width as f64,
                        self.height as f64,
                    );
                }
                (sum / (self.samples as f64)).color(GAMMA).rgb()
            })
            .collect();

        // Synchronously update the buffer
        for (x, y, pixel) in self.buffer.enumerate_pixels_mut() {
            *pixel = pixels[(y * self.width + x) as usize];
        }
    }

    /// Save the frame as is to a file.
    ///
    /// # Arugments
    /// `path` - the path to the file location
    ///
    pub fn save<P: AsRef<Path>>(self, path: P) {
        let ref mut file = File::create(path).unwrap();
        ImageRgb8(self.buffer).save(file, PNG).unwrap();
    }

    /// Helper method renders and then saves the render to a file
    pub fn render_to<P: AsRef<Path>>(&mut self, secne: &scene, path: P) {
        self.render(scene);
        self.save(path);
    }
}
