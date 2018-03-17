use rayon::prelude::*;
use std::path::Path;
use std::fs::File;
use color::Color;
use util::rand;
use shader::shader;
use scene::Scene;
use image::{ ImageBuffer, ImageRgb8, Rgb, PNG };

#[derive(Debug)]
pub struct Frame {

    pub width: u32,
    pub height: u32,
    pub samples: u32,

    coords: Vec<(u32, u32)>,
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Frame {

    pub fn new(width: u32, height: u32, samples: u32) -> Self {

        // Generate coords at construction, so it only has to be done once
        let mut coords = Vec::new();
        for y in 0..height {
            for x in 0..width {

                // Adjust for ImageBuffer y coordinates being opposite
                coords.push((x, height - y - 1));
            }
        }
        Frame {
            width:   width,
            height:  height,
            samples: samples,
            coords:  coords,
            buffer:  ImageBuffer::new(width, height),
        }
    }

    pub fn update(&mut self, scene: &Scene) {

        // In parallel shade the pixels
        let pixels: Vec<_> = self.coords.par_iter().map(|&(x, y): &(u32, u32)| {
            let mut sr = 0;
            let mut sg = 0;
            let mut sb = 0;
            for _ in 0..self.samples {
                let Color { r, g, b } = shader(
                    scene,
                    (x as f64) + rand(),
                    (y as f64) + rand(),
                    self.width as f64,
                    self.height as f64
                );
                sr = sr + (r as u32);
                sg = sg + (g as u32);
                sb = sb + (b as u32);
            }
            Color::new(
                (sr / self.samples) as u8,
                (sg / self.samples) as u8,
                (sb / self.samples) as u8
            ).rgb()
        }).collect();

        // Synchronously update the buffer
        for (x, y, pixel) in self.buffer.enumerate_pixels_mut() {
            *pixel = pixels[(y * self.width + x) as usize];
        }
    }

    pub fn save<P: AsRef<Path>>(self, path: P) {
        let ref mut file = File::create(path).unwrap();
        ImageRgb8(self.buffer).save(file, PNG).unwrap();
    }

}