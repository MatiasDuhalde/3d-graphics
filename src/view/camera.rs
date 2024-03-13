use crate::utils::{calculate_rotation_matrix, Matrix, Vector3};

pub struct Camera {
    position: Vector3,
    rotation_matrix: Matrix,
    fov: f64,
}

impl Camera {
    pub fn new(position: Vector3, rotation: Vector3, fov: f64) -> Self {
        Self {
            position,
            rotation_matrix: calculate_rotation_matrix(rotation),
            fov,
        }
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_rotation_matrix(&self) -> &Matrix {
        &self.rotation_matrix
    }

    pub fn get_fov(&self) -> f64 {
        self.fov
    }
}
