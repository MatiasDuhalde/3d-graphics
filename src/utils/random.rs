use {crate::utils::Vector3, rand::Rng, std::f64::consts::PI};

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

pub fn random_cos(vector: &Vector3) -> Vector3 {
    let r1 = random_f64();
    let r2 = random_f64();

    let u1 = 2. * PI * r1;
    let u2 = (1. - r2).sqrt();

    let x = u1.cos() * u2;
    let y = u1.sin() * u2;
    let z = r2.sqrt();

    // FIXME: Edge case when normal is (0, 0, 1)
    let t1 = vector.cross(&Vector3::new(0., 0., 1.)).normalize();
    let t2 = vector.cross(&t1).normalize();

    (t1 * x + t2 * y + *vector * z).normalize()
}
