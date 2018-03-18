use rand::distributions::{Range, Sample};
use rand::thread_rng;

/// Returns a random float between 0 and 1
pub fn rand() -> f64 {
    Range::new(0.0, 1.0).sample(&mut thread_rng())
}

/// Schlick's approximation
pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r2 = r * r;
    r2 + (1.0 - r2) * (1.0 - cosine).powf(5.0)
}
