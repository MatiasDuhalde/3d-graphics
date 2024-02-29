use {
    crate::{
        core::{Intersectable, Intersection, LightSource, Object, Ray},
        utils::{random_cos, Vector3},
    },
    std::f64::consts::PI,
};

const DEFAULT_OPAQUE: bool = false;
const DEFAULT_COLOR: Vector3 = Vector3::new(1., 1., 1.);
const DEFAULT_MIRROR: bool = false;
const DEFAULT_TRANSPARENT: bool = false;
const DEFAULT_REFRACTIVE_INDEX: f64 = 1.;
const DEFAULT_LIGHT: bool = false;
const DEFAULT_LIGHT_INTENSITY: f64 = 0.;

#[derive(Clone)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
    opaque: bool,
    color: Vector3,
    mirror: bool,
    transparent: bool,
    refractive_index: f64,
    light: bool,
    light_intensity: f64,
}

pub struct SphereBuilder {
    center: Vector3,
    radius: f64,
    opaque: bool,
    color: Vector3,
    mirror: bool,
    transparent: bool,
    refractive_index: f64,
    light: bool,
    light_intensity: f64,
}

impl SphereBuilder {
    pub fn new(center: Vector3, radius: f64) -> Self {
        SphereBuilder {
            center,
            radius,
            opaque: DEFAULT_OPAQUE,
            color: DEFAULT_COLOR,
            mirror: DEFAULT_MIRROR,
            transparent: DEFAULT_TRANSPARENT,
            refractive_index: DEFAULT_REFRACTIVE_INDEX,
            light: DEFAULT_LIGHT,
            light_intensity: DEFAULT_LIGHT_INTENSITY,
        }
    }

    pub fn with_opaque(&mut self, opaque: bool) -> &mut Self {
        self.opaque = opaque;
        self
    }

    pub fn with_color(&mut self, color: Vector3) -> &mut Self {
        self.opaque = true;
        self.color = color;
        self
    }

    pub fn with_mirror(&mut self, mirror: bool) -> &mut Self {
        self.mirror = mirror;
        self
    }

    pub fn with_transparent(&mut self, transparent: bool) -> &mut Self {
        self.transparent = transparent;
        self
    }

    pub fn with_refractive_index(&mut self, refractive_index: f64) -> &mut Self {
        self.transparent = true;
        self.refractive_index = refractive_index;
        self
    }

    pub fn with_light(&mut self, light: bool) -> &mut Self {
        self.light = light;
        self
    }

    pub fn with_light_intensity(&mut self, light_intensity: f64) -> &mut Self {
        self.light = true;
        self.light_intensity = light_intensity / (4. * PI * PI * self.radius * self.radius);
        self
    }

    pub fn build(&self) -> Sphere {
        Sphere {
            center: self.center,
            radius: self.radius,
            opaque: self.opaque,
            color: self.color,
            mirror: self.mirror,
            transparent: self.transparent,
            refractive_index: self.refractive_index,
            light: self.light,
            light_intensity: self.light_intensity,
        }
    }
}

impl Sphere {
    pub fn normal(&self, point: &Vector3) -> Vector3 {
        (*point - self.center).normalize()
    }
}

impl LightSource for Sphere {
    fn get_position(&self) -> &Vector3 {
        &self.center
    }

    fn get_intensity(&self) -> f64 {
        self.light_intensity
    }

    fn calculate_ray_from_light_source(&self, point: &Vector3) -> Ray {
        let random_direction = random_cos(&self.normal(point));
        let random_surface_point = random_direction * self.radius + self.center;

        let light_direction = (*point - random_surface_point).normalize();

        Ray::new(random_surface_point, light_direction).add_offset()
    }

    fn calculate_lambertian_shading(
        &self,
        point: &Vector3,
        normal: &Vector3,
        albedo: &Vector3,
        light_ray: &Ray,
    ) -> Vector3 {
        let light_surface_point = *light_ray.get_origin();
        let light_ray_direction = *light_ray.get_direction();
        let light_source_normal = self.normal(&light_surface_point);

        let probability_density_function =
            light_source_normal.dot(&self.normal(point)) / (PI * self.radius * self.radius);

        let a = f64::max(0., normal.dot(&-light_ray_direction));
        let b = f64::max(0., light_source_normal.dot(&light_ray_direction));

        self.light_intensity * *albedo / PI * a * b
            / ((light_surface_point - *point).norm2() * probability_density_function)
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let center_to_origin = *ray.get_origin() - self.center;
        let distance_dot = ray.get_direction().dot(&center_to_origin);
        let determinant =
            distance_dot * distance_dot - center_to_origin.norm2() + self.radius * self.radius;

        if determinant < 0. {
            return None;
        }

        let t1 = -distance_dot - determinant.sqrt();
        let t2 = -distance_dot + determinant.sqrt();

        if t2 < 0. {
            return None;
        }

        let distance = if t1 > 0. { t1 } else { t2 };

        let intersection_point = *ray.get_origin() + *ray.get_direction() * distance;

        let normal = self.normal(&intersection_point);

        Some(Intersection::new(
            intersection_point,
            normal,
            distance,
            ray.get_direction().dot(&normal) < 0.,
            Some(self),
            ray.clone(),
        ))
    }
}

impl Object for Sphere {
    fn is_opaque(&self) -> bool {
        self.opaque
    }

    fn is_mirror(&self) -> bool {
        self.mirror
    }

    fn is_transparent(&self) -> bool {
        self.transparent
    }

    fn is_light_source(&self) -> bool {
        self.light
    }

    fn get_color(&self) -> &Vector3 {
        &self.color
    }

    fn get_refractive_index(&self) -> f64 {
        self.refractive_index
    }

    fn get_light_intensity(&self) -> f64 {
        self.light_intensity
    }
}
