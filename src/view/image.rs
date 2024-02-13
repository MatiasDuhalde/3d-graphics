use {
    crate::{
        core::{Ray, Scene},
        utils::{box_muller, Vector3, ANTIALIASING_RAYS, ENABLE_ANTIALIASING, GAMMA_CORRECTION},
        view::Camera,
    },
    image::{codecs::png, ColorType, ImageEncoder},
    std::f64,
};

const COLOR_CHANNELS: u32 = 3;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
    camera: Camera,
    scene: Scene,
}

impl Image {
    pub fn new(width: u32, height: u32, camera: Camera, scene: Scene) -> Self {
        let data = vec![0; (width * height * COLOR_CHANNELS) as usize];
        Self {
            width,
            height,
            data,
            camera,
            scene,
        }
    }
}

impl Image {
    pub fn draw(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.render_pixel(i, j);
            }
        }
    }

    pub fn render_pixel(&mut self, i: u32, j: u32) {
        let ray_paths = if ENABLE_ANTIALIASING {
            ANTIALIASING_RAYS
        } else {
            1
        };

        let mut color = Vector3::new(0., 0., 0.);
        for _ in 0..ray_paths {
            let ray = if ENABLE_ANTIALIASING {
                self.calculate_random_pixel_ray(i, j)
            } else {
                self.calculate_pixel_ray(i, j)
            };
            let intersection = self.scene.intersect(&ray);
            if intersection.is_some() {
                color += self
                    .scene
                    .calculate_color(&intersection.unwrap(), ENABLE_ANTIALIASING);
            }
        }
        color /= ray_paths as f64;

        self.data[(i * self.width + j) as usize * 3 + 0] =
            f64::min(255., f64::powf(color.x(), GAMMA_CORRECTION)) as u8;
        self.data[(i * self.width + j) as usize * 3 + 1] =
            f64::min(255., f64::powf(color.y(), GAMMA_CORRECTION)) as u8;
        self.data[(i * self.width + j) as usize * 3 + 2] =
            f64::min(255., f64::powf(color.z(), GAMMA_CORRECTION)) as u8;
    }

    pub fn calculate_pixel_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_position = self.calculate_pixel_position(i, j);
        let ray_direction = (pixel_position - *self.camera.get_position()).normalize();
        Ray::new(*self.camera.get_position(), ray_direction)
    }

    pub fn calculate_random_pixel_ray(&self, i: u32, j: u32) -> Ray {
        let random_offset = box_muller(0.25);
        let random_pixel_position = self.calculate_pixel_position(i, j) + random_offset;
        let ray_direction = (random_pixel_position - *self.camera.get_position()).normalize();
        Ray::new(*self.camera.get_position(), ray_direction)
    }

    pub fn calculate_pixel_position(&self, i: u32, j: u32) -> Vector3 {
        let x = (j as f64 + 0.5) - (self.width as f64 / 2.);
        let y = -((i as f64 + 0.5) - (self.height as f64 / 2.));
        let z = -(self.width as f64 / (2. * (self.camera.get_fov() / 2.).tan()));
        Vector3::new(x, y, z) + *self.camera.get_position()
    }

    pub fn save(&self, filename: &str) {
        let file = std::fs::File::create(filename).unwrap();
        let encoder = png::PngEncoder::new(file);
        encoder
            .write_image(&self.data, self.width, self.height, ColorType::Rgb8)
            .unwrap();
    }
}
