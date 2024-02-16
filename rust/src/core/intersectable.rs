use crate::core::{Intersection, Ray};

pub trait Intersectable: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
