mod core;
mod utils;
mod view;

use {
    crate::core::{LightSource, Scene, SphereBuilder},
    crate::utils::Vector3,
    crate::view::{Camera, Image},
    std::f64::consts::PI,
};

fn main() {
    let mirror_sphere = SphereBuilder::new(Vector3::new(-25., 0., 0.), 10.)
        .with_mirror(true)
        .build();
    let solid_sphere = SphereBuilder::new(Vector3::new(25., 0., 0.), 10.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let transparent_sphere = SphereBuilder::new(Vector3::new(0., 0., 0.), 10.)
        .with_transparent(true)
        .with_refractive_index(1.5)
        .build();

    let left_sphere = SphereBuilder::new(Vector3::new(-1000., 0., 0.), 940.)
        .with_color(Vector3::new(0., 1., 1.))
        .build();
    let right_sphere = SphereBuilder::new(Vector3::new(1000., 0., 0.), 940.)
        .with_color(Vector3::new(1., 1., 0.))
        .build();
    let up_sphere = SphereBuilder::new(Vector3::new(0., 1000., 0.), 940.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let down_sphere = SphereBuilder::new(Vector3::new(0., -1000., 0.), 990.)
        .with_color(Vector3::new(0., 0., 1.))
        .build();
    let front_sphere = SphereBuilder::new(Vector3::new(0., 0., 1000.), 940.)
        .with_color(Vector3::new(0., 1., 0.))
        .build();
    let back_sphere = SphereBuilder::new(Vector3::new(0., 0., -1000.), 940.)
        .with_color(Vector3::new(1., 0., 1.))
        .build();

    let light_source = LightSource::new(Vector3::new(-10., 20., 40.), 5E9);

    let mut scene = Scene::new();

    scene
        .add_object(Box::new(mirror_sphere))
        .add_object(Box::new(solid_sphere))
        .add_object(Box::new(transparent_sphere))
        .add_object(Box::new(left_sphere))
        .add_object(Box::new(right_sphere))
        .add_object(Box::new(up_sphere))
        .add_object(Box::new(down_sphere))
        .add_object(Box::new(front_sphere))
        .add_object(Box::new(back_sphere))
        .add_light_source(Box::new(light_source));

    let camera = Camera::new(Vector3::new(0., 0., 55.), 75. * PI / 180.);

    let mut image = Image::new(512, 512, camera, scene);

    image.draw();

    image.save("output.png");
}
