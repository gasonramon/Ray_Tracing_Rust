pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.00
}

pub fn random_double() -> f64 {
    // Return a random real in [0.0, 1.0)
    rand::random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}