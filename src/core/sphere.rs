use {
    crate::{
        core::{Intersectable, Intersection, IntersectionBuilder, LightSource, Ray},
        utils::{random_cos, Vector3},
    },
    std::f64::consts::PI,
};

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
            color: DEFAULT_COLOR,
            mirror: DEFAULT_MIRROR,
            transparent: DEFAULT_TRANSPARENT,
            refractive_index: DEFAULT_REFRACTIVE_INDEX,
            light: DEFAULT_LIGHT,
            light_intensity: DEFAULT_LIGHT_INTENSITY,
        }
    }

    pub fn with_color(&mut self, color: Vector3) -> &mut Self {
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
        self.refractive_index = refractive_index;
        self
    }

    pub fn with_light(&mut self, light: bool) -> &mut Self {
        self.light = light;
        self
    }

    pub fn with_light_intensity(&mut self, light_intensity: f64) -> &mut Self {
        self.light_intensity = light_intensity / (4. * PI * PI * self.radius * self.radius);
        self
    }

    pub fn build(&self) -> Sphere {
        Sphere {
            center: self.center,
            radius: self.radius,
            color: self.color,
            mirror: self.mirror,
            transparent: self.transparent,
            refractive_index: self.refractive_index,
            light: self.light,
            light_intensity: self.light_intensity,
        }
    }
}

// impl Sphere {
//     pub fn get_center(&self) -> &Vector3 {
//         &self.center
//     }

//     pub fn get_radius(&self) -> f64 {
//         self.radius
//     }

//     pub fn get_color(&self) -> &Vector3 {
//         &self.color
//     }

//     pub fn is_mirror(&self) -> bool {
//         self.mirror
//     }

//     pub fn is_transparent(&self) -> bool {
//         self.transparent
//     }

//     pub fn get_refractive_index(&self) -> f64 {
//         self.refractive_index
//     }
// }

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

        let mut normal = self.normal(&intersection_point);

        let mut intersection_builder =
            IntersectionBuilder::new(intersection_point, normal, distance);

        if self.light {
            intersection_builder.with_light_intensity(self.light_intensity);
        } else if self.mirror {
            intersection_builder.with_reflected_ray(
                ray.calculate_reflected_ray(&intersection_point, &normal)
                    .add_offset(),
            );
        } else if self.transparent {
            // FIXME: Here we assume that the ray is exiting into the air
            let n1 = ray.get_refractive_index();
            let mut n2 = 1.0;
            if ray.get_direction().dot(&normal) < 0. {
                normal = -normal;
                n2 = self.refractive_index;
            }
            let n = n1 / n2;

            let cos_i = ray.get_direction().dot(&normal);
            let sin2_transmitted = n * n * (1. - cos_i * cos_i);

            if sin2_transmitted > 1. {
                intersection_builder.with_reflected_ray(
                    ray.calculate_reflected_ray(&intersection_point, &normal)
                        .add_offset(),
                );
            } else {
                let cos_transmitted = (1. - sin2_transmitted).sqrt();

                let refracted_normal = normal * cos_transmitted;
                let refracted_tangent = (*ray.get_direction() - normal * cos_i) * n;
                let refracted_direction = (refracted_tangent + refracted_normal).normalize();

                let refracted_ray =
                    Ray::new_with_refractive_index(intersection_point, refracted_direction, n2)
                        .add_offset();
                let reflected_ray = ray
                    .calculate_reflected_ray(&intersection_point, &normal)
                    .add_offset();

                let normal_reflection_coefficient = f64::powi((n1 - n2) / (n1 + n2), 2);
                let reflection_coefficient = normal_reflection_coefficient
                    + (1. - normal_reflection_coefficient) * f64::powi(1. - cos_i.abs(), 5);

                intersection_builder
                    .with_refracted_ray(refracted_ray)
                    .with_reflected_ray(reflected_ray)
                    .with_reflection_coefficient(reflection_coefficient);
            }
        } else {
            intersection_builder.with_albedo(self.color);
        }

        Some(intersection_builder.build())
    }
}
