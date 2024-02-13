use crate::core::{Intersection, Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
