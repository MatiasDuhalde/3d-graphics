use rand::Rng;
use std::f64::consts::PI;

use crate::utils::Vector3;

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn box_muller(sd: f64) -> Vector3 {
    let r1 = random_f64();
    let r2 = random_f64();

    let u1 = (-2. * r1.ln()).sqrt() * sd;
    let u2 = 2. * PI * r2;

    Vector3::new(u1 * u2.cos(), u1 * u2.sin(), 0.)
}
