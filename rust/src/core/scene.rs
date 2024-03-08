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

        if intersection.get_object().is_light_source() && !indirect_light {
            self.calculate_light_color(intersection)
        } else if intersection.get_object().is_opaque() {
            self.calculate_opaque_color(intersection, depth)
        } else if intersection.get_object().is_mirror() {
            self.calculate_mirror_color(intersection, depth, indirect_light)
        } else if intersection.get_object().is_transparent() {
            self.calculate_transparent_color(intersection, depth, indirect_light)
        } else {
            Vector3::new(0., 0., 0.)
        }
    }

    fn calculate_light_color(&self, intersection: &Intersection) -> Vector3 {
        Vector3::new(1., 1., 1.) * intersection.get_object().get_light_intensity()
    }

    fn calculate_opaque_color(&self, intersection: &Intersection, depth: i32) -> Vector3 {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.light_sources.len());
        let light_source = &self.light_sources[index];

        let light_ray = light_source.calculate_ray_from_light_source(intersection.get_point());
        let direct_lighting = if self.light_ray_reaches_point(&light_ray, intersection.get_point())
        {
            light_source.calculate_lambertian_shading(
                intersection.get_point(),
                intersection.get_normal(),
                &intersection.calculate_color(),
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

    fn calculate_mirror_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        self.calculate_reflected_color(intersection, depth, indirect_light)
    }

    fn calculate_reflected_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        if let Some(reflected_intersection) = self.calculate_reflected_intersection(intersection) {
            self.calculate_color_recursive(&reflected_intersection, depth + 1, indirect_light)
        } else {
            Vector3::new(0., 0., 0.)
        }
    }

    fn calculate_transparent_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        if ENABLE_FRESNEL {
            self.calculate_fresnel_color(intersection, depth, indirect_light)
        } else {
            self.calculate_refracted_color(intersection, depth, indirect_light)
        }
    }

    fn calculate_refracted_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        if let Some(refracted_intersection) = self.calculate_refracted_intersection(intersection) {
            self.calculate_color_recursive(&refracted_intersection, depth + 1, indirect_light)
        } else {
            Vector3::new(0., 0., 0.)
        }
    }

    fn light_ray_reaches_point(&self, light_ray: &Ray, point: &Vector3) -> bool {
        let intersection = self.intersect(&light_ray);

        !intersection.as_ref().is_some_and(|i| {
            RAY_OFFSET_EPSILON + i.get_distance() < (*point - *light_ray.get_origin()).norm()
        })
    }

    fn calculate_reflected_intersection(
        &self,
        intersection: &Intersection,
    ) -> Option<Intersection> {
        self.intersect(&intersection.calculate_reflected_ray())
    }

    fn calculate_refracted_intersection(
        &self,
        intersection: &Intersection,
    ) -> Option<Intersection> {
        self.intersect(&intersection.calculate_refracted_ray())
    }

    fn calculate_fresnel_color(
        &self,
        intersection: &Intersection,
        depth: i32,
        indirect_light: bool,
    ) -> Vector3 {
        let reflection_coefficient = intersection.calculate_reflection_coefficient();
        if random_f64() < reflection_coefficient {
            if let Some(reflected_intersection) =
                self.calculate_reflected_intersection(intersection)
            {
                return self.calculate_color_recursive(
                    &reflected_intersection,
                    depth + 1,
                    indirect_light,
                );
            }
        } else {
            if let Some(refracted_intersection) =
                self.calculate_refracted_intersection(intersection)
            {
                return self.calculate_color_recursive(
                    &refracted_intersection,
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
        if let Some(indirect_intersection) = self.intersect(&random_ray) {
            let color = self.calculate_color_recursive(&indirect_intersection, depth + 1, true);
            intersection
                .get_object()
                .get_color()
                .hadamard_product(&color)
        } else {
            Vector3::new(0., 0., 0.)
        }
    }

    fn calculate_random_normal_hemisphere_ray(&self, intersection: &Intersection) -> Ray {
        let random_direction = random_cos(intersection.get_normal());

        Ray::new(*intersection.get_point(), random_direction).add_offset()
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;

        for object in self.objects.iter() {
            if let Some(object_intersection) = object.intersect(ray) {
                if intersection.as_ref().map_or(true, |i| {
                    object_intersection.get_distance() < i.get_distance()
                }) {
                    intersection = Some(object_intersection);
                }
            }
        }

        intersection
    }
}
