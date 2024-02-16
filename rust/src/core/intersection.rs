use crate::{core::Ray, utils::Vector3};

const DEFAULT_ALBEDO: Option<Vector3> = None;
const DEFAULT_REFLECTED_RAY: Option<Ray> = None;
const DEFAULT_REFRACTED_RAY: Option<Ray> = None;
const DEFAULT_REFLECTION_COEFFICIENT: Option<f64> = None;
const DEFAULT_LIGHT_INTENSITY: Option<f64> = None;

pub struct Intersection {
    point: Vector3,
    normal: Vector3,
    distance: f64,
    albedo: Option<Vector3>,
    reflected_ray: Option<Ray>,
    refracted_ray: Option<Ray>,
    reflection_coefficient: Option<f64>,
    light_intensity: Option<f64>,
}

pub struct IntersectionBuilder {
    point: Vector3,
    normal: Vector3,
    distance: f64,
    albedo: Option<Vector3>,
    reflected_ray: Option<Ray>,
    refracted_ray: Option<Ray>,
    reflection_coefficient: Option<f64>,
    light_intensity: Option<f64>,
}

impl Intersection {
    pub fn get_point(&self) -> &Vector3 {
        &self.point
    }

    pub fn get_normal(&self) -> &Vector3 {
        &self.normal
    }

    pub fn get_distance(&self) -> f64 {
        self.distance
    }

    pub fn is_opaque(&self) -> bool {
        self.albedo.is_some()
    }

    pub fn get_albedo(&self) -> Option<&Vector3> {
        self.albedo.as_ref()
    }

    pub fn is_reflected(&self) -> bool {
        self.reflected_ray.is_some()
    }

    pub fn get_reflected_ray(&self) -> &Ray {
        self.reflected_ray.as_ref().unwrap()
    }

    pub fn is_refracted(&self) -> bool {
        self.refracted_ray.is_some()
    }

    pub fn get_refracted_ray(&self) -> &Ray {
        self.refracted_ray.as_ref().unwrap()
    }

    pub fn get_reflection_coefficient(&self) -> Option<f64> {
        self.reflection_coefficient
    }

    pub fn is_light(&self) -> bool {
        self.light_intensity.is_some()
    }

    pub fn get_light_intensity(&self) -> Option<f64> {
        self.light_intensity
    }
}

impl IntersectionBuilder {
    pub fn new(point: Vector3, normal: Vector3, distance: f64) -> Self {
        IntersectionBuilder {
            point,
            normal,
            distance,
            albedo: DEFAULT_ALBEDO,
            reflected_ray: DEFAULT_REFLECTED_RAY,
            refracted_ray: DEFAULT_REFRACTED_RAY,
            reflection_coefficient: DEFAULT_REFLECTION_COEFFICIENT,
            light_intensity: DEFAULT_LIGHT_INTENSITY,
        }
    }

    pub fn with_albedo(&mut self, albedo: Vector3) -> &mut Self {
        self.albedo = Some(albedo);
        self
    }

    pub fn with_reflected_ray(&mut self, reflected_ray: Ray) -> &mut Self {
        self.reflected_ray = Some(reflected_ray);
        self
    }

    pub fn with_refracted_ray(&mut self, refracted_ray: Ray) -> &mut Self {
        self.refracted_ray = Some(refracted_ray);
        self
    }

    pub fn with_reflection_coefficient(&mut self, reflection_coefficient: f64) -> &mut Self {
        self.reflection_coefficient = Some(reflection_coefficient);
        self
    }

    pub fn with_light_intensity(&mut self, light_intensity: f64) -> &mut Self {
        self.light_intensity = Some(light_intensity);
        self
    }

    pub fn build(&self) -> Intersection {
        Intersection {
            point: self.point,
            normal: self.normal,
            distance: self.distance,
            albedo: self.albedo,
            reflected_ray: self.reflected_ray,
            refracted_ray: self.refracted_ray,
            reflection_coefficient: self.reflection_coefficient,
            light_intensity: self.light_intensity,
        }
    }
}
