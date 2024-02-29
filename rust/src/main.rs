mod core;
mod utils;
mod view;

use {
    crate::{
        core::{Mesh, MeshObjectBuilder, PointLightSource, Scene, SphereBuilder},
        utils::Vector3,
        view::{Camera, Image},
    },
    std::f64::consts::PI,
};

#[allow(dead_code)]
fn spheres_image() -> Image {
    let mirror_sphere = SphereBuilder::new(Vector3::new(-25., 0., 0.), 10.)
        .with_mirror(true)
        .build();
    let solid_sphere = SphereBuilder::new(Vector3::new(25., 0., 0.), 10.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let transparent_sphere = SphereBuilder::new(Vector3::new(0., 0., 0.), 10.)
        .with_refractive_index(1.5)
        .build();
    let light_sphere = SphereBuilder::new(Vector3::new(-10., -10., 25.), 5.)
        .with_light_intensity(5E9)
        .build();

    let left_sphere = SphereBuilder::new(Vector3::new(-1000., 0., 0.), 940.)
        .with_color(Vector3::new(0., 1., 1.))
        .build();
    let right_sphere = SphereBuilder::new(Vector3::new(1000., 0., 0.), 940.)
        .with_color(Vector3::new(1., 1., 0.))
        .build();
    let up_sphere = SphereBuilder::new(Vector3::new(0., 0., 1000.), 940.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let down_sphere = SphereBuilder::new(Vector3::new(0., 0., -1000.), 990.)
        .with_color(Vector3::new(0., 0., 1.))
        .build();
    let front_sphere = SphereBuilder::new(Vector3::new(0., 1000., 0.), 940.)
        .with_color(Vector3::new(0., 1., 0.))
        .build();
    let back_sphere = SphereBuilder::new(Vector3::new(0., -1000., 0.), 940.)
        .with_color(Vector3::new(1., 0., 1.))
        .build();

    let mut scene = Scene::new();

    scene
        .add_object(Box::new(mirror_sphere))
        .add_object(Box::new(solid_sphere))
        .add_object(Box::new(transparent_sphere))
        .add_object(Box::new(light_sphere.clone()))
        .add_object(Box::new(left_sphere))
        .add_object(Box::new(right_sphere))
        .add_object(Box::new(up_sphere))
        .add_object(Box::new(down_sphere))
        .add_object(Box::new(front_sphere))
        .add_object(Box::new(back_sphere))
        .add_light_source(Box::new(light_sphere));

    let camera = Camera::new(
        Vector3::new(0., 55., 0.),
        Vector3::new(0., 0., PI),
        75. * PI / 180.,
    );

    Image::new(512, 512, camera, scene)
}

#[allow(dead_code)]
fn spinning_cat() {
    let cat_mesh = Mesh::from_obj_file("assets/cat/cat.obj");

    for theta in 0..16 {
        let cat_object = MeshObjectBuilder::new(&cat_mesh)
            .with_rotation(Vector3::new(PI / 2., 0., PI / 2. + theta as f64 * PI / 8.))
            .with_translation(Vector3::new(0., 0., -21.5))
            .with_scale(0.8)
            .with_color(Vector3::new(0.71764705882, 0.25490196078, 0.05490196078))
            .build();

        let light_source = PointLightSource::new(Vector3::new(0., 55., 0.), 5E9);

        let mut scene = Scene::new();

        scene
            .add_object(Box::new(cat_object))
            .add_light_source(Box::new(light_source));

        let camera = Camera::new(
            Vector3::new(0., 55., 0.),
            Vector3::new(0., 0., PI),
            75. * PI / 180.,
        );

        let mut image = Image::new(512, 512, camera, scene);
        image.draw();

        image.save(&format!("spinning_cat/cat_{}.png", theta));
    }
}

#[allow(dead_code)]
fn cat_image() -> Image {
    let cat_mesh = Mesh::from_obj_file("assets/cat/cat.obj");

    let cat_object = MeshObjectBuilder::new(&cat_mesh)
        .with_rotation(Vector3::new(PI / 2., 0., 0.))
        .with_translation(Vector3::new(0., 25., -15.))
        .with_scale(0.6)
        // .with_color(Vector3::new(0.71764705882, 0.25490196078, 0.05490196078))
        .with_mirror(true)
        .build();

    let light_sphere = SphereBuilder::new(Vector3::new(-10., 35., 10.), 2.5)
        .with_light(true)
        .with_light_intensity(5E9)
        .build();

    let left_sphere = SphereBuilder::new(Vector3::new(-1000., 0., 0.), 940.)
        .with_color(Vector3::new(0., 1., 1.))
        .build();
    let right_sphere = SphereBuilder::new(Vector3::new(1000., 0., 0.), 940.)
        .with_color(Vector3::new(1., 1., 0.))
        .build();
    let up_sphere = SphereBuilder::new(Vector3::new(0., 0., 1000.), 940.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let down_sphere = SphereBuilder::new(Vector3::new(0., 0., -1000.), 990.)
        .with_color(Vector3::new(0., 0., 1.))
        .build();
    let front_sphere = SphereBuilder::new(Vector3::new(0., 1000., 0.), 940.)
        .with_color(Vector3::new(0., 1., 0.))
        .build();
    let back_sphere = SphereBuilder::new(Vector3::new(0., -1000., 0.), 940.)
        .with_color(Vector3::new(1., 0., 1.))
        .build();

    let mut scene = Scene::new();

    scene
        .add_object(Box::new(cat_object))
        .add_object(Box::new(left_sphere))
        .add_object(Box::new(right_sphere))
        .add_object(Box::new(up_sphere))
        .add_object(Box::new(down_sphere))
        .add_object(Box::new(front_sphere))
        .add_object(Box::new(back_sphere))
        .add_object(Box::new(light_sphere.clone()))
        .add_light_source(Box::new(light_sphere));

    let camera = Camera::new(
        Vector3::new(0., 55., 0.),
        Vector3::new(0., 0., PI),
        75. * PI / 180.,
    );

    Image::new(512, 512, camera, scene)
}

fn main() {
    let mut image = cat_image();
    image.draw();
    image.save("cat.png");

    // spinning_cat();

    // let mut image = spheres_image();
    // image.draw();
    // image.save("spheres.png");
}
