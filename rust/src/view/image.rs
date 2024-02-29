use {
    crate::{
        core::{Intersectable, Ray, Scene},
        utils::{
            box_muller, Vector3, ANTIALIASING_RAYS, ENABLE_ANTIALIASING, ENABLE_FRESNEL,
            ENABLE_INDIRECT_LIGHTING, FRESNEL_RAYS, GAMMA_CORRECTION, INDIRECT_LIGHTING_RAYS,
        },
        view::Camera,
    },
    image::{codecs::png, ColorType, ImageEncoder},
    rayon::{
        iter::{IndexedParallelIterator, ParallelIterator},
        slice::ParallelSliceMut,
    },
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
        let data = self.calculate_pixel_values();
        self.data.copy_from_slice(&data);
    }

    pub fn calculate_pixel_values(&self) -> Vec<u8> {
        let mut data = vec![0; (self.width * self.height * COLOR_CHANNELS) as usize];

        data.par_chunks_mut((self.width * COLOR_CHANNELS) as usize)
            .enumerate()
            .for_each(|(i, row)| {
                for (j, pixel) in (0..row.len()).step_by(COLOR_CHANNELS as usize).enumerate() {
                    let color = self.calculate_pixel_color(i, j);
                    row[pixel] = color.0;
                    row[pixel + 1] = color.1;
                    row[pixel + 2] = color.2;
                }
            });

        data
    }

    fn calculate_pixel_color(&self, i: usize, j: usize) -> (u8, u8, u8) {
        let ray_paths = if ENABLE_ANTIALIASING {
            ANTIALIASING_RAYS
        } else if ENABLE_FRESNEL {
            FRESNEL_RAYS
        } else if ENABLE_INDIRECT_LIGHTING {
            INDIRECT_LIGHTING_RAYS
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

            if let Some(intersection) = self.scene.intersect(&ray) {
                color += self.scene.calculate_color(&intersection);
            }
        }
        color /= ray_paths as f64;

        self.gamma_correct_color(color)
    }

    fn gamma_correct_color(&self, color: Vector3) -> (u8, u8, u8) {
        (
            f64::min(255., f64::powf(color.x(), GAMMA_CORRECTION)) as u8,
            f64::min(255., f64::powf(color.y(), GAMMA_CORRECTION)) as u8,
            f64::min(255., f64::powf(color.z(), GAMMA_CORRECTION)) as u8,
        )
    }

    fn calculate_pixel_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_position = self.calculate_pixel_position(i, j);
        let ray_direction = (pixel_position - *self.camera.get_position()).normalize();
        Ray::new(*self.camera.get_position(), ray_direction)
    }

    fn calculate_random_pixel_ray(&self, i: usize, j: usize) -> Ray {
        let random_offset = box_muller(0.25);
        let random_pixel_position = self.calculate_pixel_position(i, j) + random_offset;
        let ray_direction = (random_pixel_position - *self.camera.get_position()).normalize();
        Ray::new(*self.camera.get_position(), ray_direction)
    }

    fn calculate_pixel_position(&self, i: usize, j: usize) -> Vector3 {
        let camera_position = *self.camera.get_position();

        let fov_scale = f64::tan(self.camera.get_fov() / 2.);

        let x = -(j as f64 + 0.5) + (self.width as f64 / 2.);
        let y = self.width as f64 / (2. * fov_scale);
        let z = -((i as f64 + 0.5) - (self.height as f64 / 2.));

        let pixel_position = Vector3::new(x, y, z);

        let rotation_matrix = *self.camera.get_rotation_matrix();

        rotation_matrix * pixel_position + camera_position
    }

    pub fn save(&self, filename: &str) {
        let file = std::fs::File::create(filename)
            .unwrap_or_else(|err| panic!("Error creating file {}: {}", filename, err));

        let encoder = png::PngEncoder::new(file);

        encoder
            .write_image(&self.data, self.width, self.height, ColorType::Rgb8)
            .unwrap_or_else(|err| panic!("Error writing image to file {}: {}", filename, err));
    }
}
