extern crate image;
extern crate time;
extern crate rayon;

extern crate rusttracer;

use std::path::Path;
use std::env;

use rusttracer::math::Vector3;
use rusttracer::{Scene, Color, Camera, MaterialTemplate, Renderer, SuperSampling};
use rusttracer::geometry::{Shape, Sphere, Plane};
use rusttracer::lights::PointLight;

const MAX_DEPTH: u32 = 5;
const WIDTH: u32 = 2560;
const HEIGHT: u32 = 1440;

fn main() {
    let benchmark = match env::var("BENCHMARK") {
        Ok(s) => s.parse::<bool>().unwrap_or(false),
        Err(_) => false,
    };
    let template = MaterialTemplate::new(Color::blue() * 0.02,
                                         Color::black(),
                                         Color::black(),
                                         None,
                                         None);

    let floor_material = template.build_material(|material| {
                                                     material.ambient_color = Color::white() * 0.05;
                                                     material.diffuse_color = Color::white() * 0.3;
                                                 });
    let floor = Plane::new(Vector3::new(0.0, -5.0, 0.0),
                           Vector3::new(0.0, 1.0, 0.0),
                           floor_material);

    let back_material =
        template.build_material(|material| { material.diffuse_color = Color::white() * 0.8; });
    let back = Plane::new(Vector3::new(0.0, 0.0, 50.0),
                          Vector3::new(0.0, 0.0, -1.0),
                          back_material);

    let m1 = template.build_material(|material| { material.reflection_coefficient = Some(1.0); });
    let s1 = Sphere::new(Vector3::new(0.0, -4.0, 45.0), 1.0, m1);

    let l1 = PointLight::new(Vector3::new(0.0, 10.0, 45.0), Color::new(67, 249, 253), 0.4);
    let l2 = PointLight::new(Vector3::new(-15.0, 10.0, 45.0),
                             Color::new(92, 253, 67),
                             0.4);
    let l3 = PointLight::new(Vector3::new(15.0, 10.0, 45.0), Color::new(253, 115, 6), 0.4);

    let objects: Vec<&Shape> = vec![&s1, &floor, &back];
    let lights: Vec<&PointLight> = vec![&l1, &l2, &l3];
    let scene = Scene::new(&objects, &lights, Color::black());
    let camera = Camera::new(0.873,
                             WIDTH,
                             HEIGHT,
                             Vector3::new(-10.0, 5.0, 10.0),
                             s1.origin,
                             Vector3::new(0.0, 1.0, 0.0));

    let renderer = Renderer::new(&scene, &camera, SuperSampling::On(2));

    if benchmark {
        for _ in 0..10 {
            let _ = renderer.render(MAX_DEPTH);
        }

        return;
    }

    let buffer = renderer.render(MAX_DEPTH);

    let timestamp = time::get_time().sec;
    let filename = format!("images/{}.png", timestamp);
    image::save_buffer(&Path::new(&filename),
                       &buffer[..],
                       WIDTH,
                       HEIGHT,
                       image::RGB(8))
            .unwrap();
}
