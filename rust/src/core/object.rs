use crate::{core::Intersectable, utils::Vector3};

pub trait Object: Sync + Send + Intersectable {
    fn is_opaque(&self) -> bool;

    fn is_mirror(&self) -> bool;

    fn is_transparent(&self) -> bool;

    fn is_light_source(&self) -> bool;

    fn get_color(&self) -> &Vector3;

    fn get_refractive_index(&self) -> f64;

    fn get_light_intensity(&self) -> f64;
}
