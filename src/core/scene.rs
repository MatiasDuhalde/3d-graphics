use {
    crate::{
        core::{Intersectable, Intersection, LightSource, Ray},
        utils::{
            random_cos, random_f64, Vector3, ENABLE_FRESNEL, ENABLE_INDIRECT_LIGHTING,
            MAX_RECURSION_DEPTH, RAY_OFFSET_EPSILON,
        },
    },
    rand::Rng,
};

pub struct Scene {
    objects: Vec<Box<dyn Intersectable>>,
    light_sources: Vec<Box<dyn LightSource>>,
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

    pub fn add_light_source(&mut self, light_source: Box<dyn LightSource>) -> &mut Self {
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

    pub fn calculate_color(&self, intersection: &Intersection) -> Vector3 {
        if self.light_sources.is_empty() {
            Vector3::new(0., 0., 0.)
        } else {
            self.calculate_color_recursive(intersection, 1, false)
        }
    }

    fn calculate_color_recursive(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        if depth > MAX_RECURSION_DEPTH {
            return Vector3::new(0., 0., 0.);
        }

        if intersection.is_light() && !indirect_light {
            self.calculate_light_color(intersection)
        } else if intersection.is_opaque() {
            self.calculate_opaque_color(intersection, depth)
        } else if intersection.is_refracted() {
            // a refracted intersection is also reflected, so this goes first
            self.calculate_refracted_color(intersection, depth, indirect_light)
        } else if intersection.is_reflected() {
            self.calculate_reflected_color(intersection, depth, indirect_light)
        } else {
            Vector3::new(0., 0., 0.)
        }
    }

    fn calculate_light_color(&self, intersection: &Intersection) -> Vector3 {
        Vector3::new(1., 1., 1.) * intersection.get_light_intensity().unwrap()
    }

    fn calculate_opaque_color(&self, intersection: &Intersection, depth: i32) -> Vector3 {
        // Pick random light source
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.light_sources.len());
        let light_source = &self.light_sources[index];

        let light_ray = light_source.calculate_ray_from_light_source(intersection.get_point());
        let direct_lighting = if self.light_ray_reaches_point(&light_ray, intersection.get_point())
        {
            light_source.calculate_lambertian_shading(
                intersection.get_point(),
                intersection.get_normal(),
                intersection.get_albedo().unwrap(),
                &light_ray,
            )
        } else {
            Vector3::new(0., 0., 0.)
        };

        if ENABLE_INDIRECT_LIGHTING {
            direct_lighting + self.calculate_indirect_lighting_color(intersection, depth)
        } else {
            direct_lighting
        }
    }

    fn calculate_refracted_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        if ENABLE_FRESNEL {
            return self.calculate_fresnel_color(intersection, depth, indirect_light);
        }
        let refracted_intersection = self.calculate_refracted_intersection(intersection);
        if refracted_intersection.is_some() {
            self.calculate_color_recursive(
                &refracted_intersection.unwrap(),
                depth + 1,
                indirect_light,
            )
        } else {
            Vector3::new(0., 0., 0.)
        }
    }

    fn calculate_reflected_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        let reflected_intersection = self.calculate_reflected_intersection(intersection);
        if reflected_intersection.is_some() {
            self.calculate_color_recursive(
                &reflected_intersection.unwrap(),
                depth + 1,
                indirect_light,
            )
        } else {
            Vector3::new(0., 0., 0.)
        }
    }

    fn light_ray_reaches_point(&self, light_ray: &Ray, point: &Vector3) -> bool {
        let intersection = self.intersect(&light_ray);
        intersection.is_none()
            || RAY_OFFSET_EPSILON
                >= (*point - *light_ray.get_origin()).norm() - intersection.unwrap().get_distance()
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
        indirect_light: bool,
    ) -> Vector3 {
        if random_f64() < intersection.get_reflection_coefficient().unwrap() {
            let reflected_intersection = self.calculate_reflected_intersection(intersection);
            if reflected_intersection.is_some() {
                return self.calculate_color_recursive(
                    &reflected_intersection.unwrap(),
                    depth + 1,
                    indirect_light,
                );
            }
        } else {
            let refracted_intersection = self.calculate_refracted_intersection(intersection);
            if refracted_intersection.is_some() {
                return self.calculate_color_recursive(
                    &refracted_intersection.unwrap(),
                    depth + 1,
                    indirect_light,
                );
            }
        }

        Vector3::new(0., 0., 0.)
    }

    fn calculate_indirect_lighting_color(
        &self,
        intersection: &Intersection,
        depth: i32,
    ) -> Vector3 {
        let random_ray = self.calculate_random_normal_hemisphere_ray(intersection);
        let indirect_intersection = self.intersect(&random_ray);
        if indirect_intersection.is_some() {
            let color =
                self.calculate_color_recursive(&indirect_intersection.unwrap(), depth + 1, true);

            return intersection.get_albedo().unwrap().hadamard_product(&color);
        }

        Vector3::new(0., 0., 0.)
    }

    fn calculate_random_normal_hemisphere_ray(&self, intersection: &Intersection) -> Ray {
        let random_direction = random_cos(intersection.get_normal());

        Ray::new(*intersection.get_point(), random_direction).add_offset()
    }
}
