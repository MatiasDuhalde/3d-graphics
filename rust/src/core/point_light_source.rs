use {
    crate::{
        core::{LightSource, Ray},
        utils::Vector3,
    },
    std::f64::consts::PI,
};

pub struct PointLightSource {
    position: Vector3,
    intensity: f64,
}

impl PointLightSource {
    pub fn new(position: Vector3, intensity: f64) -> Self {
        PointLightSource {
            position,
            intensity,
        }
    }
}

impl LightSource for PointLightSource {
    fn get_position(&self) -> &Vector3 {
        &self.position
    }

    fn get_intensity(&self) -> f64 {
        self.intensity
    }

    fn calculate_ray_from_light_source(&self, point: &Vector3) -> Ray {
        Ray::new(self.position, (*point - self.position).normalize()).add_offset()
    }

    fn calculate_lambertian_shading(
        &self,
        point: &Vector3,
        normal: &Vector3,
        albedo: &Vector3,
        _light_ray: &Ray,
    ) -> Vector3 {
        let light_direction = self.position - *point;
        let d2 = light_direction.norm2();

        let surface_power = self.intensity / (4. * PI * d2);

        *albedo * surface_power * f64::max(0., normal.dot(&light_direction.normalized())) / PI
    }
}
