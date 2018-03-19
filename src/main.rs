#[macro_use]
extern crate serde_derive;
extern crate toml;

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
mod config;
mod aabb;
mod texture;
mod noise;

use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::time::SystemTime;
use config::Config;

fn main() {
    // Load configuration
    let config_path = "./rust-tracer.toml";
    print!("\nLoading Configuration...");
    let config = match File::open(config_path) {
        // Load the configuration file
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file!");
            toml::from_str(contents.as_ref())
                .expect("Failed to deserealize config!")
        },
        Err(_) => {
            // Write the default configuration to file
            let config   = Config::default();
            let mut file = File::create(config_path).unwrap();
            file.write_all(toml::to_string(&config)
                .expect("Failed to serealize config!")
                .as_bytes()
            ).expect("Failed to wrie file!");
            config
        },
    };
    println!("done.\n{}", config);

    // Run and time configuration
    let start = SystemTime::now();
    {
        config.run();
    }
    let end = SystemTime::now();
    println!("The render took {}s.", end
        .duration_since(start)
        .unwrap()
        .as_secs()
    );
}