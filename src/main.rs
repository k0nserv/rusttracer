extern crate image;
extern crate time;

extern crate rusttracer;

use std::path::Path;
use std::env;

use rusttracer::math::Vector3;
use rusttracer::{Scene, Color, Camera, Material, Renderer, SuperSampling};
use rusttracer::geometry::{Shape, Sphere, Plane};

const MAX_DEPTH: u32 = 5;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
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

    let floor_material = Material::new(Color::white(), Color::black(), 0.0);
    let floor = Plane::new(Vector3::new(0.0, -15.0, 0.0),
                           Vector3::new(0.0, 1.0, 0.0),
                           floor_material);

    let m1 = Material::new(Color::new(29, 86, 140), Color::black(), 0.0);
    let s1 = Sphere::new(Vector3::new(0.0, 0.0, 15.0), 1.0, m1);

    let m2 = Material::new(Color::new(140, 10, 29), Color::black(), 0.0);
    let s2 = Sphere::new(Vector3::new(-3.0, 0.0, 15.0), 1.0, m2);

    let m3 = Material::new(Color::new(10, 145, 29), Color::black(), 0.0);
    let s3 = Sphere::new(Vector3::new(3.0, 0.0, 15.0), 1.0, m3);

    let m4 = Material::new(Color::new(10, 145, 120), Color::black(), 0.0);
    let s4 = Sphere::new(Vector3::new(1.5, 2.0, 15.0), 0.5, m4);

    let m5 = Material::new(Color::new(190, 145, 29), Color::black(), 0.0);
    let s5 = Sphere::new(Vector3::new(-1.5, 2.0, 15.0), 0.5, m5);

    let m6 = Material::new(Color::new(150, 10, 120), Color::black(), 0.0);
    let s6 = Sphere::new(Vector3::new(1.5, -2.0, 15.0), 0.5, m6);

    let m7 = Material::new(Color::new(190, 20, 29), Color::black(), 0.0);
    let s7 = Sphere::new(Vector3::new(-1.5, -2.0, 15.0), 0.5, m7);

    let objects: Vec<&Shape> = vec![&s1, &s2, &s3, &s4, &s5, &s6, &s7, &floor];
    let scene = Scene::new(&objects, Color::black());
    let camera = Camera::new(0.785398163, WIDTH, HEIGHT);

    let renderer = Renderer::new(&scene, &camera, SuperSampling::On(2), num_threads);


    if benchmark {
        for i in 0..10 {
            let x: Vec<Color> = renderer.render(MAX_DEPTH);
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
