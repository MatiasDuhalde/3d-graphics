mod core;
mod utils;
mod view;

use {
    crate::{
        core::{Mesh, MeshObjectBuilder, PointLightSource, Scene, SphereBuilder},
        utils::Vector3,
        view::{Camera, Image},
    },
    core::Texture,
    std::{f64::consts::PI, time::Instant},
};

fn add_walls(scene: &mut Scene) -> &mut Scene {
    // Teal sphere
    let left_sphere = SphereBuilder::new(Vector3::new(-1000., 0., 0.), 940.)
        .with_color(Vector3::new(0., 1., 1.))
        .build();
    // Yellow Sphere
    let right_sphere = SphereBuilder::new(Vector3::new(1000., 0., 0.), 940.)
        .with_color(Vector3::new(1., 1., 0.))
        .build();
    // Red Sphere
    let top_sphere = SphereBuilder::new(Vector3::new(0., 0., 1000.), 940.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    // Blue Sphere
    let bottom_sphere = SphereBuilder::new(Vector3::new(0., 0., -1000.), 990.)
        .with_color(Vector3::new(0., 0., 1.))
        .build();
    // Purple Sphere
    let front_sphere = SphereBuilder::new(Vector3::new(0., -1000., 0.), 940.)
        .with_color(Vector3::new(1., 0., 1.))
        .build();
    // Green Sphere
    let back_sphere = SphereBuilder::new(Vector3::new(0., 1000., 0.), 940.)
        .with_color(Vector3::new(0., 1., 0.))
        .build();

    scene
        .add_object(Box::new(left_sphere))
        .add_object(Box::new(right_sphere))
        .add_object(Box::new(top_sphere))
        .add_object(Box::new(bottom_sphere))
        .add_object(Box::new(back_sphere))
        .add_object(Box::new(front_sphere))
}

fn time_function(f: fn() -> ()) -> u128 {
    let start_time = Instant::now();

    f();

    start_time.elapsed().as_millis()
}

fn benchmark(f: fn() -> (), n: u32) {
    let mut times: Vec<u128> = vec![0; n as usize];

    for i in 0..n {
        println!("Running iteration {}", i + 1);
        let time = time_function(f);
        times.push(time);
        println!("Time: {} ms", time);
    }

    let average_time = times.iter().sum::<u128>() / n as u128;
    println!("Average time: {} ms", average_time);
}

#[allow(dead_code)]
fn basic_spheres_demo() {
    let white_sphere = SphereBuilder::new(Vector3::new(0., 0., 0.), 10.)
        .with_color(Vector3::new(1., 1., 1.))
        .build();

    let mut scene = Scene::new();

    let point_light_source = PointLightSource::new(Vector3::new(20., 20., 20.), 5E9);

    scene
        .add_object(Box::new(white_sphere))
        .add_light_source(Box::new(point_light_source));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(0., 55., 0.),
        Vector3::new(0., 0., PI),
        75. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/basic_spheres_demo.png");
}

#[allow(dead_code)]
fn mirror_glass_demo() {
    let mirror_sphere = SphereBuilder::new(Vector3::new(-25., 0., 0.), 10.)
        .with_mirror(true)
        .build();
    let red_sphere = SphereBuilder::new(Vector3::new(25., 0., 0.), 10.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let transparent_sphere = SphereBuilder::new(Vector3::new(0., 0., 0.), 10.)
        .with_refractive_index(1.5)
        .build();

    let mut scene = Scene::new();

    let point_light_source = PointLightSource::new(Vector3::new(20., 20., 20.), 5E9);

    scene
        .add_object(Box::new(mirror_sphere))
        .add_object(Box::new(red_sphere))
        .add_object(Box::new(transparent_sphere))
        .add_light_source(Box::new(point_light_source));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(0., 55., 0.),
        Vector3::new(0., 0., PI),
        75. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/mirror_glass_spheres_demo.png");
}

#[allow(dead_code)]
fn fresnel_demo() {
    let transparent_sphere = SphereBuilder::new(Vector3::new(0., 0., 10.), 15.)
        .with_refractive_index(1.5)
        .build();
    let mirror_sphere = SphereBuilder::new(Vector3::new(25., 0., 0.), 10.)
        .with_mirror(true)
        .build();
    let red_sphere = SphereBuilder::new(Vector3::new(-25., 15., 20.), 5.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();

    let mut scene = Scene::new();

    let point_light_source = PointLightSource::new(Vector3::new(20., 20., 30.), 5E9);

    scene
        .add_object(Box::new(mirror_sphere))
        .add_object(Box::new(red_sphere))
        .add_object(Box::new(transparent_sphere))
        .add_light_source(Box::new(point_light_source));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(30., 20., 5.),
        Vector3::new(0., PI / 8., 3. * PI / 4.),
        90. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/fresnel_demo.png");
}

#[allow(dead_code)]
fn indirect_lighting_demo() {
    let red_sphere = SphereBuilder::new(Vector3::new(0., 10., 0.), 10.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let pink_sphere = SphereBuilder::new(Vector3::new(0., 50., 0.), 10.)
        .with_color(Vector3::new(1., 0.5, 0.5))
        .build();
    let orange_sphere = SphereBuilder::new(Vector3::new(-20., 30., 0.), 10.)
        .with_color(Vector3::new(1., 0.5, 0.))
        .build();
    let yellow_sphere = SphereBuilder::new(Vector3::new(20., 30., 0.), 10.)
        .with_color(Vector3::new(1., 1., 0.))
        .build();

    let mut scene = Scene::new();

    let point_light_source = PointLightSource::new(Vector3::new(0., 30., 0.), 5E9);

    scene
        .add_object(Box::new(red_sphere))
        .add_object(Box::new(pink_sphere))
        .add_object(Box::new(orange_sphere))
        .add_object(Box::new(yellow_sphere))
        .add_light_source(Box::new(point_light_source));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(25., 55., 20.),
        Vector3::new(-PI / 4., 0., 3. * PI / 4.),
        90. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/indirect_lighting_demo.png");
}

#[allow(dead_code)]
fn antialiasing_demo() {
    let red_sphere = SphereBuilder::new(Vector3::new(20., -40., 0.), 10.)
        .with_color(Vector3::new(1., 0., 0.))
        .build();
    let glass_sphere = SphereBuilder::new(Vector3::new(0., 0., 0.), 10.)
        .with_refractive_index(1.5)
        .build();
    let mirror_sphere = SphereBuilder::new(Vector3::new(-40., 20., 0.), 10.)
        .with_mirror(true)
        .build();

    let white_sphere = SphereBuilder::new(Vector3::new(40., 10., 0.), 10.)
        .with_color(Vector3::new(1., 1., 1.))
        .build();
    let purple_sphere = SphereBuilder::new(Vector3::new(10., 40., 0.), 10.)
        .with_color(Vector3::new(1., 0., 1.))
        .build();

    let mut scene = Scene::new();

    let point_light_source = PointLightSource::new(Vector3::new(-30., -30., 40.), 5E9);

    scene
        .add_object(Box::new(red_sphere))
        .add_object(Box::new(glass_sphere))
        .add_object(Box::new(mirror_sphere))
        .add_object(Box::new(white_sphere))
        .add_object(Box::new(purple_sphere))
        .add_light_source(Box::new(point_light_source));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(50., 50., 40.),
        Vector3::new(-PI / 4., 0., 3. * PI / 4.),
        90. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/antialiasing_demo.png");
}

#[allow(dead_code)]
fn spherical_lights_demo() {
    let glass_sphere = SphereBuilder::new(Vector3::new(-20., 10., 0.), 10.)
        .with_refractive_index(1.5)
        .build();
    let mirror_sphere = SphereBuilder::new(Vector3::new(20., 10., 0.), 10.)
        .with_mirror(true)
        .build();
    let light_sphere = SphereBuilder::new(Vector3::new(0., -30., 15.), 5.)
        .with_light_intensity(5E9)
        .build();
    let light_sphere2 = SphereBuilder::new(Vector3::new(0., 40., 25.), 5.)
        .with_light_intensity(5E9)
        .build();

    let mut scene = Scene::new();

    scene
        .add_object(Box::new(glass_sphere))
        .add_object(Box::new(mirror_sphere))
        .add_object(Box::new(light_sphere.clone()))
        .add_object(Box::new(light_sphere2.clone()))
        .add_light_source(Box::new(light_sphere))
        .add_light_source(Box::new(light_sphere2));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(50., 50., 40.),
        Vector3::new(-PI / 4., 0., 3. * PI / 4.),
        75. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/spherical_lights_demo.png");
}

#[allow(dead_code)]
fn meshes_demo() {
    let cat_mesh = Mesh::from_obj_file("assets/cat/cat.obj");

    let mut builder = MeshObjectBuilder::new(&cat_mesh);
    builder
        .with_rotation(Vector3::new(PI / 2., 0., 0.))
        .with_translation(Vector3::new(-10., 25., -15.))
        .with_scale(0.6)
        .with_mirror(true);

    let cat_object = builder.build();

    let mut builder2 = MeshObjectBuilder::new(&cat_mesh);
    builder2
        .with_rotation(Vector3::new(-PI / 2., 0., PI))
        .with_translation(Vector3::new(10., 25., 50.))
        .with_scale(0.6)
        .with_refractive_index(1.5);

    let cat_object2 = builder2.build();

    let light_source = PointLightSource::new(Vector3::new(0., 30., 20.), 5E9);

    let mut scene = Scene::new();

    scene
        .add_object(Box::new(cat_object))
        .add_object(Box::new(cat_object2))
        .add_light_source(Box::new(light_source));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(0., 55., 10.),
        Vector3::new(0., 0., PI),
        75. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/meshes_demo.png");
}

#[allow(dead_code)]
fn mesh_normals_and_texture_mapping_demo() {
    let cat_obj_file = "assets/cat/cat.obj";

    let cat_mesh = Mesh::from_obj_file(cat_obj_file);
    let cat_texture = Texture::from_obj_file(cat_obj_file);

    let mut builder = MeshObjectBuilder::new(&cat_mesh);
    builder
        .with_rotation(Vector3::new(PI / 2., 0., -PI))
        .with_translation(Vector3::new(20., 20., -15.))
        .with_scale(0.6)
        .with_texture(cat_texture);

    let cat_object = builder.build();

    let intensity = 5E9;

    let light_sphere = SphereBuilder::new(Vector3::new(10., 25., 10.), 5.)
        .with_light_intensity(intensity)
        .build();

    let mirror_sphere = SphereBuilder::new(Vector3::new(-25., 20., 5.), 15.)
        .with_mirror(true)
        .build();

    let mut scene = Scene::new();

    scene
        .add_object(Box::new(cat_object))
        .add_object(Box::new(mirror_sphere))
        .add_object(Box::new(light_sphere.clone()))
        .add_light_source(Box::new(light_sphere));

    add_walls(&mut scene);

    let camera = Camera::new(
        Vector3::new(0., 35., 5.),
        Vector3::new(0., 0., 7. * PI / 8.),
        90. * PI / 180.,
    );

    let mut image = Image::new(512, 512, camera, scene);
    image.draw();
    image.save("figures/mesh_normals_and_texture_mapping_demo.png");
}

#[allow(dead_code)]
fn spinning_cat() {
    let cat_mesh = Mesh::from_obj_file("assets/cat/cat.obj");

    for theta in 0..16 {
        let mut builder = MeshObjectBuilder::new(&cat_mesh);

        builder
            .with_rotation(Vector3::new(PI / 2., 0., PI / 2. + theta as f64 * PI / 8.))
            .with_translation(Vector3::new(0., 0., -21.5))
            .with_scale(0.8)
            .with_color(Vector3::new(0.71764705882, 0.25490196078, 0.05490196078));

        let cat_object = builder.build();

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

fn main() {
    // benchmark(basic_spheres_demo, 5);
    // benchmark(mirror_glass_demo, 5);
    // benchmark(fresnel_demo, 1);
    // benchmark(indirect_lighting_demo, 1);
    // benchmark(antialiasing_demo, 1);
    // benchmark(spherical_lights_demo, 1);
    // benchmark(meshes_demo, 1);
    benchmark(mesh_normals_and_texture_mapping_demo, 1);
}
