use crate::utils::Vector3;

pub struct Camera {
    position: Vector3,
    rotation: Vector3,
    fov: f64,
}

impl Camera {
    pub fn new(position: Vector3, rotation: Vector3, fov: f64) -> Self {
        Self {
            position,
            rotation,
            fov,
        }
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_rotation(&self) -> &Vector3 {
        &self.rotation
    }

    pub fn get_fov(&self) -> f64 {
        self.fov
    }
}
