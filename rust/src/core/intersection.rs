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
        let n2: f64 = self.calculate_n2();
        let normal = self.calculate_inside_normal();

        self.source_ray
            .calculate_refracted_ray(&self.point, &normal, n2)
            .add_offset()
    }

    pub fn calculate_reflection_coefficient(&self) -> f64 {
        let normal = self.calculate_inside_normal();

        let cos_i: f64 = self.source_ray.get_direction().dot(&normal);

        let normal_reflection_coefficient = self.calculate_normal_reflection_coefficient();

        normal_reflection_coefficient
            + (1. - normal_reflection_coefficient) * f64::powi(1. - cos_i.abs(), 5)
    }

    fn calculate_normal_reflection_coefficient(&self) -> f64 {
        let n1 = self.source_ray.get_refractive_index();
        let n2: f64 = self.calculate_n2();

        f64::powi((n1 - n2) / (n1 + n2), 2)
    }

    fn calculate_n2(&self) -> f64 {
        // FIXME: Here we assume that the ray is exiting into the air
        if self.exterior {
            self.object.map_or(1., |obj| obj.get_refractive_index())
        } else {
            1.
        }
    }

    fn calculate_inside_normal(&self) -> Vector3 {
        if self.exterior {
            -self.normal
        } else {
            self.normal
        }
    }
}
