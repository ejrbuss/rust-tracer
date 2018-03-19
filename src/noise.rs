use util::rand;
use vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Perlin {
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
    rand_float: Vec<f64>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
            rand_float: perlin_generate(),
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        // let u = point.x - point.x.floor();
        // let v = point.y - point.y.floor();
        // let w = point.z - point.z.floor();
        let i = ((4.0 * point.x) as usize) & 255;
        let j = ((4.0 * point.y) as usize) & 255;
        let k = ((4.0 * point.z) as usize) & 255;
        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn perlin_generate() -> Vec<f64> {
    let mut v = Vec::new();
    for _ in 0..256 {
        v.push(rand());
    }
    v
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut v = Vec::new();
    for i in 0..256 {
        v.push(i as usize);
    }
    permute(&mut v, 256);
    v
}

fn permute(v: &mut Vec<usize>, n: i32) {
    for i in (1..n).rev() {
        let target = (rand() * ((i as f64) + 1.0)) as usize;
        v.swap(i as usize, target);
    }
}
