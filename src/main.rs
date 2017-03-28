extern crate image;
extern crate time;

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
const BUFFER_SIZE: usize = (WIDTH * HEIGHT * 3) as usize;
const DEFAULT_NUMBER_OF_THREADS: u32 = 4;

fn main() {
    let num_threads = match env::var("NUM_THREADS") {
        Ok(threads) => threads.parse::<u32>().unwrap_or(DEFAULT_NUMBER_OF_THREADS),
        Err(_) => DEFAULT_NUMBER_OF_THREADS,
    };

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

    let renderer = Renderer::new(&scene, &camera, SuperSampling::On(2), num_threads);

    if benchmark {
        for _ in 0..10 {
            let _ = renderer.render(MAX_DEPTH);
        }

        return;
    }

    let result: Vec<Color> = renderer.render(MAX_DEPTH);
    let mut buffer = vec![0x8C; BUFFER_SIZE];

    for (index, pixel) in result.iter().enumerate() {
        buffer[(index * 3) + 0] = pixel.r();
        buffer[(index * 3) + 1] = pixel.g();
        buffer[(index * 3) + 2] = pixel.b();
    }

    let timestamp = time::get_time().sec;
    let filename = format!("images/{}.png", timestamp);
    image::save_buffer(&Path::new(&filename),
                       &buffer[..],
                       WIDTH,
                       HEIGHT,
                       image::RGB(8))
            .unwrap();
}
