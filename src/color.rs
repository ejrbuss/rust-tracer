extern crate image;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new Color isntance
    ///
    /// # Arguments
    /// * `r` - the red channel
    /// * `g` - the green channel
    /// * `b` - the blue channel
    ///
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Create a new Color instances from floats given a gamma value. The gamma
    /// value is applied by taking each channgel to the power of gamma.
    ///
    /// # Arguments
    /// * `r`     - the red channel
    /// * `g`     - the green channel
    /// * `b`     - the blue channel
    /// * `gamma` - the gamma value
    ///
    pub fn fnew(r: f64, g: f64, b: f64, gamma: f64) -> Self {
        Color {
            r: (r.powf(gamma) * 255.99) as u8,
            g: (g.powf(gamma) * 255.99) as u8,
            b: (b.powf(gamma) * 255.99) as u8,
        }
    }

    /// Get the color black
    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    /// Get the color white
    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }

    /// Get the color red
    pub fn red() -> Self {
        Color::new(255, 0, 0)
    }

    /// Get the color green
    pub fn green() -> Self {
        Color::new(0, 255, 0)
    }

    /// Get the color blue
    pub fn blue() -> Self {
        Color::new(0, 0, 255)
    }

    /// Get a Pixel value in black and white
    pub fn bw(&self) -> image::Rgb<u8> {
        let c = (((self.r as f64)
                + (self.g as f64)
                + (self.b as f64)
        ) / 3.0) as u8;
        image::Rgb { data: [c, c, c] }
    }

    /// Get a Pixel value in red green and blue
    pub fn rgb(&self) -> image::Rgb<u8> {
        image::Rgb {
            data: [self.r, self.g, self.b],
        }
    }
}
