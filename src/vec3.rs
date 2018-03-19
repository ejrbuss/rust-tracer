use color::Color;

use std::ops::{Add, Div, Mul, Neg, Sub, Index};
use util::rand;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Creates a new vector with components (x, y, z)
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Creates a new zero vector (0, 0, 0)
    pub fn zeros() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    /// Creates a new ones vector (1, 1, 1)
    pub fn ones() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    /// Returns the dot product of vectors u and v
    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Returns the cross product of vectors u and v
    pub fn cross(u: Vec3, v: Vec3) -> Self {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            -(u.x * v.z - u.z * v.x),
            u.x * v.y - u.y * v.x,
        )
    }

    /// Returns the reflection of vector u around normal v
    pub fn reflect(u: Vec3, v: Vec3) -> Self {
        u - 2.0 * Vec3::dot(u, v) * v
    }

    /// Returns the refraction of vector v pased the normal vector n if
    /// possible. k represents ni/nt.
    pub fn refract(v: Vec3, n: Vec3, k: f64) -> Option<Self> {
        let uv = v.unit();
        let dt = Vec3::dot(uv, n);
        let d = 1.0 - k * k * (1.0 - dt * dt);
        if d > 0.0 {
            Some(k * (uv - n * dt) - n * d.sqrt())
        } else {
            None
        }
    }

    /// Returns a random vector within the unit sphere
    pub fn rand() -> Self {
        loop {
            let v = 2.0 * Vec3::new(rand(), rand(), rand()) - Vec3::ones();
            if v.mag() < 1.0 {
                return v;
            }
        }
    }

    /// Returns a racomd vector within the unit disc
    pub fn rand_disc() -> Self {
        loop {
            let v = 2.0 * Vec3::new(rand(), rand(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
            if Vec3::dot(v, v) < 1.0 {
                return v;
            }
        }
    }

    /// Returns the magnitude of the vector, ie. its dot product with itself
    pub fn mag(self) -> f64 {
        Vec3::dot(self, self)
    }

    /// Returns the length of the vector
    pub fn len(self) -> f64 {
        self.mag().sqrt()
    }

    /// Returns an equivalent unit vector
    pub fn unit(self) -> Self {
        self / self.len()
    }

    /// Converts the current vector to a color given a specific gamma correction
    pub fn color(&self, gamma: f64) -> Color {
        Color::fnew(self.x, self.y, self.z, gamma)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Index<u32> for Vec3 {
    type Output = f64;
    fn index(&self, i: u32) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("unexpected vector index {}!", i),
        }
    }
}

// Macro for generating binary vector operators
macro_rules! vec3_binary_op {
    ($trait:ident, $func:ident, $op:tt) => (

        // Vec3 op Vec3
        impl $trait<Vec3> for Vec3 {
            type Output = Vec3;
            fn $func(self, v: Vec3) -> Vec3 {
                Vec3::new(self.x $op v.x, self.y $op v.y, self.z $op v.z)
            }
        }

        // Vec3 op f64
        impl $trait<f64> for Vec3 {
            type Output = Vec3;
            fn $func(self, f: f64) -> Vec3 {
                Vec3::new(self.x $op f, self.y $op f, self.z $op f)
            }
        }

        // f64 op Vec3
        impl $trait<Vec3> for f64 {
            type Output = Vec3;
            fn $func(self, v: Vec3) -> Vec3 {
                Vec3::new(self $op v.x, self $op v.y, self $op v.z)
            }
        }
    )
}

// Supported binary vector operators
vec3_binary_op!(Add, add, +);
vec3_binary_op!(Sub, sub, -);
vec3_binary_op!(Mul, mul, *);
vec3_binary_op!(Div, div, /);
