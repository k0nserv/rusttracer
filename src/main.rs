#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
extern crate getopts;
extern crate image;
extern crate time;

extern crate rusttracer;

use std::path::Path;
use std::env;

use getopts::Options;

use rusttracer::{Camera, Color, Config, IllumninationModel, Material, MaterialTemplate, Renderer,
                 Scene, SuperSampling};
use rusttracer::mesh_loader::MeshLoader;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt(
        "c",
        "config-path",
        "config file path, uses `default.json` if not specified",
        "CONFIG_PATH",
    );
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
        return;
    }

    let benchmark = matches.opt_present("b");
    let config_path = matches.opt_str("c").expect("No config provided");

    let config = Config::new_from_file(&config_path).expect("Invalid configuration");

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
    let floor_material = template.build_material(|material| {
        material.ambient_color = Color::white() * 0.05;
        material.diffuse_color = Color::white() * 0.6;
    });

    let materials = config
        .materials
        .iter()
        .map(|material_config| Material::new_from_config(material_config))
        .collect();
    let scene = Scene::new_from_config(
        config.scenes.first().unwrap(),
        &materials,
        &mesh_loader,
        floor_material,
    ).expect("Invalid scene");
    let camera_config = config.cameras.first().unwrap();
    let camera = Camera::from_config(camera_config);
    let renderer = Renderer::new(&scene, &camera, SuperSampling::On(2));

    if benchmark {
        for _ in 0..10 {
            let _ = renderer.render(config.max_depth);
        }

        return;
    }

    let buffer = renderer.render(config.max_depth);

    let timestamp = time::get_time().sec;
    let filename = format!("images/{}.png", timestamp);
    image::save_buffer(
        &Path::new(&filename),
        &buffer[..],
        camera_config.width,
        camera_config.height,
        image::RGB(8),
    ).unwrap();
}
