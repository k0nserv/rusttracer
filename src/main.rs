#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate getopts;
extern crate image;
extern crate time;

extern crate rusttracer;

use std::path::Path;
use std::env;
use std::f64::consts::PI;

use getopts::Options;

use rusttracer::math::{Matrix4, Point3, Transform, Vector3};
use rusttracer::{Camera, Color, Config, MaterialTemplate, Renderer, Scene, SuperSampling,
                 IllumninationModel};
use rusttracer::geometry::{Intersectable, Plane, Transformable, Sphere};
use rusttracer::lights::PointLight;
use rusttracer::mesh_loader::MeshLoader;

const DEFAULT_CONFIG_PATH: &str = "default.toml";

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("c",
                "config-path",
                "config file path, uses `default.toml` if not specified",
                "CONFIG_PATH");
    opts.optflag("b",
                 "benchmark",
                 "Benchmark by rendering the scene multiple times");
    opts.optflag("h", "help", "prints this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let benchmark = matches.opt_present("b");

    let config_path = matches.opt_str("c").unwrap_or(DEFAULT_CONFIG_PATH.to_string());

    let config = Config::new_from_file(&config_path).expect("Invalid configuration");

    let template = MaterialTemplate::new(Color::blue() * 0.02,
                                         Color::black(),
                                         Color::black(),
                                         0.0,
                                         IllumninationModel::DiffuseSpecular,
                                         None,
                                         None);

    let floor_material = template.build_material(|material| {
                                                     material.ambient_color = Color::white() * 0.05;
                                                     material.diffuse_color = Color::white() * 0.6;
                                                 });
    let floor = Plane::new(Point3::new(0.0, -5.0, 0.0),
                           Vector3::new(0.0, 1.0, 0.0),
                           floor_material);

    let back_material =
        template.build_material(|material| { material.diffuse_color = Color::white() * 0.8; });
    let back = Plane::new(Point3::new(0.0, 0.0, -50.0),
                          Vector3::new(0.0, 0.0, 1.0),
                          back_material);

    let l1 = PointLight::new(Point3::new(0.0, 3.0, -45.0), Color::new(67, 249, 253), 5.0);
    let l2 = PointLight::new(Point3::new(-15.0, 3.0, -45.0), Color::new(92, 253, 67), 5.0);
    let l3 = PointLight::new(Point3::new(15.0, 3.0, -45.0), Color::new(253, 115, 6), 5.0);
    let l4 = PointLight::new(Point3::new(0.0, 10.0, -20.0), Color::white(), 100.0);

    let white_material = template.build_material(|material| {
                                                     material.diffuse_color = Color::white() * 0.6;
                                                     material.specular_color = Color::white();
                                                     material.specular_exponent = 90.0;
                                                     material.reflection_coefficient = Some(0.3);
                                                 });

    /*
    let mesh_loader = MeshLoader::new();
    let mut suzanne = mesh_loader.load(Path::new("models/CornellBox-Mirror-Blender.obj"), &white_material);


    for (index, mesh) in suzanne.iter_mut().enumerate() {
        let transform = Transform::new(
            Matrix4::scale_uniform(3.0) * Matrix4::rot_y(0.0) *
                Matrix4::translate(-0.0, -5.0, -40.0),
        );
        mesh.transform(&transform);
        objects.push(mesh as &Intersectable);
    }
    */

    let sphere_template = MaterialTemplate::new(Color::blue() * 0.02,
                                                Color::red() * 0.7,
                                                Color::white() * 0.5,
                                                50.0,
                                                IllumninationModel::DiffuseSpecular,
                                                Some(0.3),
                                                Some(1.3));


    let s1_material = sphere_template.build_material(|material| {
                                                         material.illumination_model =
                                                             IllumninationModel::Constant;
                                                     });
    let s1 = Sphere::new(Point3::new(-7.0, -3.0, -40.0), 1.0, s1_material);

    let s2_material = sphere_template.build_material(|material| {
                                                         material.illumination_model =
                                                             IllumninationModel::Diffuse;
                                                     });
    let s2 = Sphere::new(Point3::new(-4.5, -3.0, -40.0), 1.0, s2_material);

    let s3_material = sphere_template.build_material(|material| {
                                                         material.illumination_model =
                                                             IllumninationModel::DiffuseSpecular;
                                                     });
    let s3 = Sphere::new(Point3::new(-2.0, -3.0, -40.0), 1.0, s3_material);

    let s4_material = sphere_template.build_material(|material| {
                                                         material.illumination_model =
                                               IllumninationModel::DiffuseSpecularReflective;
                                                     });
    let s4 = Sphere::new(Point3::new(0.5, -3.0, -40.0), 1.0, s4_material);

    let s5_material = sphere_template.build_material(|material| {
                                                         material.illumination_model =
                                               IllumninationModel::DiffuseSpecularFresnel;
                                                     });
    let s5 = Sphere::new(Point3::new(3.0, -3.0, -40.0), 1.0, s4_material);

    let mut objects: Vec<&Intersectable> = vec![&floor, &back, &s1, &s2, &s3, &s4, &s5];
    let lights: Vec<&PointLight> = vec![&l1, &l2, &l3, &l4];
    let scene = Scene::new(&objects, &lights, Color::black());
    let camera = Camera::new(0.873,
                             config.width,
                             config.height,
                             Point3::new(0.0, 0.0, -20.0),
                             Point3::new(0.0, -3.0, -40.0),
                             Vector3::new(0.0, 1.0, 0.0));

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
    image::save_buffer(&Path::new(&filename),
                       &buffer[..],
                       config.width,
                       config.height,
                       image::RGB(8))
            .unwrap();
}
