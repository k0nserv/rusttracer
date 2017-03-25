extern crate image;

extern crate rusttracer;

use std::path::Path;

use rusttracer::math::Vector3;
use rusttracer::{Scene, Color, Camera, Material, Renderer};
use rusttracer::geometry::{Shape, Sphere};

const MAX_DEPTH: u32 = 5;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const BUFFER_SIZE: usize = (WIDTH * HEIGHT * 3) as usize;

fn main() {
    let material = Material::new(Color::red(), Color::black(), 0.0);
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, 15.0), 1.0, material);
    let objects: Vec<&Shape> = vec![&sphere];
    let scene = Scene::new(&objects, Color::black());
    let camera = Camera::new(0.785398163, WIDTH, HEIGHT);

    let renderer = Renderer::new(&scene, &camera);

    let result: Vec<Color> = renderer.render(MAX_DEPTH);

    let mut buffer: [u8; BUFFER_SIZE] = [0x8C; BUFFER_SIZE];

    for (index, pixel) in result.iter().enumerate() {
        buffer[(index * 3) + 0] = pixel.r();
        buffer[(index * 3) + 1] = pixel.g();
        buffer[(index * 3) + 2] = pixel.b();
    }




    image::save_buffer(&Path::new("image.png"),
                       &buffer[..],
                       WIDTH,
                       HEIGHT,
                       image::RGB(8))
            .unwrap();
}
