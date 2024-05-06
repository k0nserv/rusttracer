#[macro_use]
mod math;
mod config_loader;
mod intersection;
mod light;
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
pub use self::config_loader::ConfigLoader;
pub use self::material::{IllumninationModel, Material, MaterialTemplate};
pub use self::renderer::{Renderer, SuperSampling};
pub use self::scene::Scene;
