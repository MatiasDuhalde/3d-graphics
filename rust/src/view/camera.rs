use crate::utils::{Matrix, Vector3};

pub struct Camera {
    position: Vector3,
    rotation_matrix: Matrix,
    fov: f64,
}

impl Camera {
    pub fn new(position: Vector3, rotation: Vector3, fov: f64) -> Self {
        Self {
            position,
            rotation_matrix: Matrix::new(
                Vector3::new(1., 0., 0.),
                Vector3::new(0., f64::cos(rotation.x()), -f64::sin(rotation.x())),
                Vector3::new(0., f64::sin(rotation.x()), f64::cos(rotation.x())),
            ) * Matrix::new(
                Vector3::new(f64::cos(rotation.y()), 0., f64::sin(rotation.y())),
                Vector3::new(0., 1., 0.),
                Vector3::new(-f64::sin(rotation.y()), 0., f64::cos(rotation.y())),
            ) * Matrix::new(
                Vector3::new(f64::cos(rotation.z()), -f64::sin(rotation.z()), 0.),
                Vector3::new(f64::sin(rotation.z()), f64::cos(rotation.z()), 0.),
                Vector3::new(0., 0., 1.),
            ),
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
