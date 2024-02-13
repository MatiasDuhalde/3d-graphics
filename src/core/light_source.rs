use crate::{core::Ray, utils::Vector3};

pub trait LightSource: Sync + Send {
    fn get_position(&self) -> &Vector3;
    fn get_intensity(&self) -> f64;
    fn calculate_ray_from_light_source(&self, point: &Vector3) -> Ray;
    fn calculate_lambertian_shading(
        &self,
        point: &Vector3,
        normal: &Vector3,
        albedo: &Vector3,
        light_ray: &Ray,
    ) -> Vector3;
}
