use vec3::Vec3;
use geo::Intersection;
use noise::Perlin;

#[derive(Debug, Clone)]
pub enum Texture {
    /// Texture representation of solid colors
    Solid {
        color: Vec3,
    },
    /// Texture representation of a checker pattern
    Checker {
        odd: Box<Texture>,
        even: Box<Texture>,
        frequency: f64,
    },
    /// Noise texture
    Noise {
        perlin: Box<Perlin>,
    },
}

impl Texture {
    /// Creates a new solid color texture
    pub fn solid(color: Vec3) -> Self {
        Texture::Solid { color }
    }

    /// Creates a new checker texture
    pub fn checker(odd: Texture, even: Texture, frequency: f64) -> Self {
        Texture::Checker {
            odd:  Box::new(odd),
            even: Box::new(even),
            frequency,
        }
    }

    /// Creates a mew noise texture
    pub fn noise() -> Self {
        Texture::Noise { perlin: Box::new(Perlin::new()) }
    }

    /// Gets the value at a given coordinate
    pub fn value(&self, i: &Intersection) -> Vec3 {
        match self {
            // Handle texture value for solid colors (trivial)
            &Texture::Solid { color } => color,
            // Handle texture value for checker
            &Texture::Checker { ref odd, ref even, frequency } => {
                let sines = (frequency * i.point.x).sin()
                          * (frequency * i.point.y).sin()
                          * (frequency * i.point.z).sin();
                if sines < 0.0 { odd.value(i) } else { even.value(i) }
            }
            // Handle noise texture
            &Texture::Noise { ref perlin } => Vec3::ones() * perlin.noise(i.point),
        }
    }
}