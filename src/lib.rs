#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(try_from)]
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod math;
mod intersection;
mod lights;
mod ray;

pub mod camera;
pub mod color;
mod config;
pub mod geometry;
pub mod material;
pub mod mesh_loader;
pub mod renderer;
pub mod scene;
pub mod texture;

pub use self::camera::Camera;
pub use self::color::Color;
pub use self::config::Config;
pub use self::material::{IllumninationModel, Material, MaterialTemplate};
pub use self::renderer::{Renderer, SuperSampling};
pub use self::scene::Scene;
