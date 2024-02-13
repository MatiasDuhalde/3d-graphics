use crate::utils::Vector3;

pub struct LightSource {
    position: Vector3,
    intensity: f64,
}

impl LightSource {
    pub fn new(position: Vector3, intensity: f64) -> Self {
        LightSource {
            position,
            intensity,
        }
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_intensity(&self) -> f64 {
        self.intensity
    }
}
