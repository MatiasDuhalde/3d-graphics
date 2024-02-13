use {
    crate::{
        core::{Intersectable, Intersection, LightSource, Ray},
        utils::{
            random_f64, Vector3, ENABLE_FRESNEL, ENABLE_INDIRECT_LIGHTING, FRESNEL_RAYS,
            INDIRECT_LIGHTING_RAYS, MAX_RECURSION_DEPTH,
        },
    },
    std::f64::consts::PI,
};

pub struct Scene {
    objects: Vec<Box<dyn Intersectable>>,
    light_sources: Vec<Box<LightSource>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            light_sources: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Intersectable>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn add_light_source(&mut self, light_source: Box<LightSource>) -> &mut Self {
        self.light_sources.push(light_source);
        self
    }
}

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;

        for object in self.objects.iter() {
            let object_intersection = object.intersect(ray);
            if object_intersection.is_some() {
                let intersection_reference = object_intersection.as_ref().unwrap();
                if intersection.is_none()
                    || intersection_reference.get_distance()
                        < intersection.as_ref().unwrap().get_distance()
                {
                    intersection = object_intersection;
                }
            }
        }

        intersection
    }

    pub fn calculate_color(&self, intersection: &Intersection, multi_sampling: bool) -> Vector3 {
        if self.light_sources.is_empty() {
            Vector3::new(0., 0., 0.)
        } else {
            self.calculate_color_recursive(intersection, 1, multi_sampling)
        }
    }

    fn calculate_color_recursive(
        &self,
        intersection: &Intersection,
        depth: i32,
        multi_sampling: bool,
    ) -> Vector3 {
        if depth > MAX_RECURSION_DEPTH {
            return Vector3::new(0., 0., 0.);
        }

        if intersection.is_opaque() {
            let light_source = self.light_sources[0].as_ref();
            let direct_lighting =
                if self.light_source_reaches_point(intersection.get_point(), light_source) {
                    self.calculate_lambertian_shading(light_source, intersection)
                } else {
                    Vector3::new(0., 0., 0.)
                };

            if ENABLE_INDIRECT_LIGHTING {
                return direct_lighting
                    + self.calculate_indirect_lighting_color(intersection, depth, multi_sampling);
            }
            return direct_lighting;
        }

        // a refracted intersection is also reflected, so this goes first
        if intersection.is_refracted() {
            if ENABLE_FRESNEL {
                return self.calculate_fresnel_color(intersection, depth, multi_sampling);
            }
            let refracted_intersection = self.calculate_refracted_intersection(intersection);
            if refracted_intersection.is_some() {
                return self.calculate_color_recursive(
                    &refracted_intersection.unwrap(),
                    depth + 1,
                    multi_sampling,
                );
            }
        }

        if intersection.is_reflected() {
            let reflected_intersection = self.calculate_reflected_intersection(intersection);
            if reflected_intersection.is_some() {
                return self.calculate_color_recursive(
                    &reflected_intersection.unwrap(),
                    depth + 1,
                    multi_sampling,
                );
            }
        }

        Vector3::new(0., 0., 0.)
    }

    fn light_source_reaches_point(&self, point: &Vector3, light_source: &LightSource) -> bool {
        let light_direction = (*light_source.get_position() - *point).normalize();

        let ray_to_light_source = Ray::new(*point, light_direction).add_offset();

        let intersection = self.intersect(&ray_to_light_source);
        if intersection.is_some() {
            return intersection.unwrap().get_distance()
                >= (*light_source.get_position() - *point).norm();
        }

        true
    }

    fn calculate_lambertian_shading(
        &self,
        light_source: &LightSource,
        intersection: &Intersection,
    ) -> Vector3 {
        let mut light_direction = *light_source.get_position() - *intersection.get_point();
        let d2 = light_direction.norm2();

        let surface_power = light_source.get_intensity() / (4. * PI * d2);

        *intersection.get_albedo().unwrap()
            * surface_power
            * f64::max(
                0.,
                intersection.get_normal().dot(&light_direction.normalize()),
            )
            / PI
    }

    fn calculate_reflected_intersection(
        &self,
        intersection: &Intersection,
    ) -> Option<Intersection> {
        self.intersect(intersection.get_reflected_ray())
    }

    fn calculate_refracted_intersection(
        &self,
        intersection: &Intersection,
    ) -> Option<Intersection> {
        self.intersect(intersection.get_refracted_ray())
    }

    fn calculate_fresnel_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        multi_sampling: bool,
    ) -> Vector3 {
        let ray_paths = if multi_sampling { 1 } else { FRESNEL_RAYS };

        let reflection_coefficient = intersection.get_reflection_coefficient().unwrap();

        let mut color = Vector3::new(0., 0., 0.);
        for _ in 0..ray_paths {
            if random_f64() < reflection_coefficient {
                let reflected_intersection = self.calculate_reflected_intersection(intersection);
                if reflected_intersection.is_some() {
                    color += self.calculate_color_recursive(
                        &reflected_intersection.unwrap(),
                        depth + 1,
                        true,
                    );
                }
            } else {
                let refracted_intersection = self.calculate_refracted_intersection(intersection);
                if refracted_intersection.is_some() {
                    color += self.calculate_color_recursive(
                        &refracted_intersection.unwrap(),
                        depth + 1,
                        true,
                    );
                }
            }
        }

        color / ray_paths as f64
    }

    fn calculate_indirect_lighting_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        multi_sampling: bool,
    ) -> Vector3 {
        let ray_paths = if multi_sampling {
            1
        } else {
            INDIRECT_LIGHTING_RAYS
        };

        let mut color = Vector3::new(0., 0., 0.);
        for _ in 0..ray_paths {
            let random_ray = self.calculate_random_normal_hemisphere_ray(intersection);
            let indirect_intersection = self.intersect(&random_ray);
            if indirect_intersection.is_some() {
                color += self.calculate_color_recursive(
                    &indirect_intersection.unwrap(),
                    depth + 1,
                    true,
                );
            }
        }

        color / ray_paths as f64
    }

    fn calculate_random_normal_hemisphere_ray(&self, intersection: &Intersection) -> Ray {
        let r1 = random_f64();
        let r2 = random_f64();

        let x = (2. * PI * r1).cos() * (1. - r2).sqrt();
        let y = (2. * PI * r1).sin() * (1. - r2).sqrt();
        let z = r2.sqrt();

        // FIXME: Edge case when normal is (0, 0, 1)
        let t1 = intersection
            .get_normal()
            .cross(&Vector3::new(0., 0., 1.))
            .normalize();
        let t2 = intersection.get_normal().cross(&t1).normalize();

        let direction = (t1 * x + t2 * y + *intersection.get_normal() * z).normalize();

        Ray::new(*intersection.get_point(), direction).add_offset()
    }
}
