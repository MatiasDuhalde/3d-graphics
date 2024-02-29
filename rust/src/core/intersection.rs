use crate::{
    core::{Object, Ray},
    utils::Vector3,
};

pub struct Intersection<'a> {
    point: Vector3,
    normal: Vector3,
    distance: f64,
    exterior: bool,
    object: Option<&'a dyn Object>,
    source_ray: Ray,
}

impl<'a> Intersection<'a> {
    pub fn new(
        point: Vector3,
        normal: Vector3,
        distance: f64,
        exterior: bool,
        object: Option<&'a dyn Object>,
        source_ray: Ray,
    ) -> Intersection {
        Intersection {
            point,
            normal,
            exterior,
            distance,
            object,
            source_ray,
        }
    }

    pub fn get_point(&self) -> &Vector3 {
        &self.point
    }

    pub fn get_normal(&self) -> &Vector3 {
        &self.normal
    }

    pub fn get_distance(&self) -> f64 {
        self.distance
    }

    pub fn is_exterior(&self) -> bool {
        self.exterior
    }

    pub fn get_object(&self) -> &dyn Object {
        self.object.unwrap()
    }

    pub fn get_source_ray(&self) -> &Ray {
        &self.source_ray
    }

    pub fn set_object(&mut self, object: &'a dyn Object) {
        self.object = Some(object);
    }

    pub fn calculate_reflected_ray(&self) -> Ray {
        self.source_ray
            .calculate_reflected_ray(&self.point, &self.normal)
            .add_offset()
    }

    pub fn calculate_refracted_ray(&self) -> Ray {
        // FIXME: Here we assume that the ray is exiting into the air
        let mut n2: f64 = 1.;
        let mut normal = self.normal;
        if self.exterior {
            normal = -normal;
            n2 = self.object.map_or(1., |obj| obj.get_refractive_index());
        }

        self.source_ray
            .calculate_refracted_ray(&self.point, &normal, n2)
            .add_offset()
    }

    pub fn calculate_reflection_coefficient(&self) -> f64 {
        let n1 = self.source_ray.get_refractive_index();

        // FIXME: Here we assume that the ray is exiting into the air
        let mut n2 = 1.;
        let mut normal = self.normal;
        if self.exterior {
            normal = -normal;
            n2 = self.object.map_or(1., |obj| obj.get_refractive_index());
        }

        let cos_i = self.source_ray.get_direction().dot(&normal);

        let normal_reflection_coefficient = f64::powi((n1 - n2) / (n1 + n2), 2);
        normal_reflection_coefficient
            + (1. - normal_reflection_coefficient) * f64::powi(1. - cos_i.abs(), 5)
    }
}
