#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
extern crate getopts;
extern crate image;
extern crate time;

extern crate rusttracer;

use std::env;
use std::f32::consts::PI;
use std::path::Path;
use std::rc::Rc;

use getopts::Options;

#[cfg(feature = "stats")]
use rusttracer::geometry::triangle::stats;
use rusttracer::mesh_loader::MeshLoader;
use rusttracer::texture;
use rusttracer::{Camera, Color, Config, IllumninationModel, Material, MaterialTemplate, Renderer,
                 Scene, SuperSampling};

#[cfg(feature = "stats")]
fn print_triangle_stats() {
    let number_of_tests = stats::number_of_triangle_intersections();
    let number_of_hits = stats::number_of_triangle_hits();

    println!("Total number of ray-triangle tests: {}", number_of_tests);
    println!("Total number of ray-triangle hits: {}", number_of_hits);
    println!(
        "Efficiency: {:.5}%",
        (f64::from(number_of_hits as u32) / f64::from(number_of_tests as u32)) * 100.0
    );
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("c", "config-path", "config file path", "CONFIG_PATH");
    opts.optflag(
        "b",
        "benchmark",
        "Benchmark by rendering the scene multiple times",
    );
    opts.optflag("h", "help", "prints this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return Ok(());
    }

    let benchmark = matches.opt_present("b");
    let config_path = matches.opt_str("c").expect("No config provided");

    let config = Config::new_from_file(&config_path)?;

    let scene_path = Path::new(&config_path).parent().unwrap();
    let mesh_loader = MeshLoader::new(scene_path.to_path_buf());
    let template = MaterialTemplate::new(
        Color::blue() * 0.02,
        Color::black(),
        Color::black(),
        0.0,
        IllumninationModel::DiffuseSpecular,
        None,
        None,
    );
    let fallback_material = Rc::new(template.build_material(|material| {
        material.ambient_color = Color::white() * 0.05;
        material.diffuse_color = Color::white() * 0.0;
        material.ambient_texture = Some(Box::new(texture::Procedural::new(|uv| {
            Color::new_f32(
                ((uv.x * 32.0 * PI).sin() + (uv.y * 32.0 * PI).cos() + 1.0) * 0.5,
                0.0,
                0.0,
            )
        })));
    }));

    let materials = config
        .materials
        .iter()
        .map(|material_config| {
            (
                material_config.name.to_owned(),
                Rc::new(Material::new_from_config(material_config)),
            )
        })
        .collect();
    let scene = Scene::new_from_config(
        &config.scenes.first().unwrap(),
        &materials,
        &mesh_loader,
        fallback_material,
    )?;
    let camera_config = config
        .cameras
        .first()
        .expect("Config should contain at least one valid camera");
    let camera = Camera::from(camera_config);
    let renderer = Renderer::new(&scene, &camera, SuperSampling::On(2));

    if benchmark {
        for _ in 0..10 {
            let _ = renderer.render(config.max_depth);
        }

        return Ok(());
    }

    let buffer = renderer.render(config.max_depth);

    #[cfg(feature = "stats")]
    print_triangle_stats();

    let timestamp = time::get_time().sec;
    let filename = format!("images/{}.png", timestamp);
    image::save_buffer(
        &Path::new(&filename),
        &buffer[..],
        camera_config.width,
        camera_config.height,
        image::RGB(8),
    ).unwrap();

    Ok(())
}
