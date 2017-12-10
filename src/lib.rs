#![feature(try_from)]
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod math;
mod geometry;
mod ray;
mod lights;
mod intersection;

pub mod color;
pub mod material;
pub mod scene;
pub mod camera;
pub mod renderer;
pub mod mesh_loader;
mod config;

pub use self::scene::Scene;
pub use self::color::Color;
pub use self::camera::Camera;
pub use self::material::{IllumninationModel, Material, MaterialTemplate};
pub use self::renderer::{Renderer, SuperSampling};
pub use self::config::Config;
