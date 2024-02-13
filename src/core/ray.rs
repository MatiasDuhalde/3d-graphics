// ray struct

use crate::utils::{Vector3, RAY_OFFSET_EPSILON};

const DEFAULT_REFRACTIVE_INDEX: f64 = 1.;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    refractive_index: f64,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            origin,
            direction,
            refractive_index: DEFAULT_REFRACTIVE_INDEX,
        }
    }

    pub fn new_with_refractive_index(
        origin: Vector3,
        direction: Vector3,
        refractive_index: f64,
    ) -> Ray {
        Ray {
            origin,
            direction,
            refractive_index,
        }
    }

    pub fn get_origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn add_offset(&mut self) -> Self {
        self.origin += self.direction * RAY_OFFSET_EPSILON;
        *self
    }

    pub fn calculate_reflected_ray(&self, intersection_point: &Vector3, normal: &Vector3) -> Self {
        let reflected_direction = self.direction - 2. * *normal * self.direction.dot(normal);
        Ray::new_with_refractive_index(
            *intersection_point,
            reflected_direction,
            self.refractive_index,
        )
    }
}
