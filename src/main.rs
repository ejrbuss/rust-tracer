extern crate image;
extern crate rand;
extern crate rayon;

mod util;
mod color;
mod frame;
mod vec3;
mod ray;
mod shader;
mod camera;
mod geo;
mod scene;
mod material;
mod scenes;

use frame::Frame;
use scenes::random_scene;

/// Configuration varibles
const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1000;
const SAMPLES: u32 = 100;
const ASPECT: f64 = (WIDTH as f64) / (HEIGHT as f64);

fn main() {
    Frame::new(WIDTH, HEIGHT, SAMPLES)
        .render_to(&random_scene(ASPECT), "out.png")
}