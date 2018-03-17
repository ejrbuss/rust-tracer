extern crate image;

use std::ops::{ Add, Div };

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn fnew(r: f64, g: f64, b: f64, gamma: f64) -> Self {
        Color {
            r: (r.powf(gamma) * 255.99) as u8,
            g: (g.powf(gamma) * 255.99) as u8,
            b: (b.powf(gamma) * 255.99) as u8,
        }
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }

    pub fn red() -> Self {
        Color::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Color::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Color::new(0, 0, 255)
    }

    pub fn bw(&self) -> image::Rgb<u8> {
        let c = (((self.r as f64) +
                  (self.g as f64) +
                  (self.b as f64)) / 3.0) as u8;
        image::Rgb { data: [ c, c, c ] }
    }

    pub fn rgb(&self) -> image::Rgb<u8> {
        image::Rgb { data: [ self.r, self.g, self.b ] }
    }

}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, c: Color) -> Color {
        Color::new(self.r + c.r, self.g + c.g, self.b + c.b)
    }
}

impl Div<u8> for Color {
    type Output = Color;
    fn div(self, f: u8) -> Color {
        Color::new(self.r / f, self.g / f, self.b / f)
    }
}