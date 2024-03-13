use {
    crate::utils::{Vector3, GAMMA_CORRECTION},
    std::fs,
};

pub struct Texture {
    data: Vec<Vector3>,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn from_obj_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Failed to read file");
        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 0 {
                continue;
            }
            match parts[0] {
                "mtllib" => {
                    let mtl_relative_filename = parts[1];
                    let mtl_filename = filename
                        .replace(filename.split('/').last().unwrap(), mtl_relative_filename);
                    return Texture::from_mtl_file(&mtl_filename);
                }
                _ => {}
            }
        }

        panic!("Could not find texture reference in obj file")
    }

    pub fn from_mtl_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Failed to read file");
        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 0 {
                continue;
            }
            match parts[0] {
                "map_Kd" => {
                    let texture_relative_filename = parts[1];
                    let texture_filename = filename.replace(
                        filename.split('/').last().unwrap(),
                        texture_relative_filename,
                    );
                    return Texture::from_img_file(&texture_filename);
                }
                _ => {}
            }
        }

        panic!("Could not find texture reference in mtl file")
    }

    pub fn from_img_file(filename: &str) -> Self {
        let texture = image::open(filename).expect("Failed to open texture");
        let width = texture.width() as usize;
        let height = texture.height() as usize;
        let data = texture
            .to_rgb8()
            .pixels()
            .map(|p| {
                let channels = p.0;
                Vector3::new(
                    (channels[0] as f64 / 255.).powf(1. / GAMMA_CORRECTION),
                    (channels[1] as f64 / 255.).powf(1. / GAMMA_CORRECTION),
                    (channels[2] as f64 / 255.).powf(1. / GAMMA_CORRECTION),
                )
            })
            .collect();

        Texture {
            data,
            width,
            height,
        }
    }

    pub fn get_color(&self, point: &Vector3) -> Vector3 {
        let x = (point.x() * self.width as f64) as usize;
        let y = self.height - (point.y() * self.height as f64) as usize;
        self.data[y * self.width + x]
    }
}
