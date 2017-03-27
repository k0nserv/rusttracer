#[macro_use]
pub mod math;
pub mod ray;
pub mod geometry;
pub mod lights;
pub mod intersection;
pub mod color;
pub mod scene;
pub mod camera;
pub mod material;
pub mod renderer;

pub use self::scene::Scene;
pub use self::color::Color;
pub use self::ray::Ray;
pub use self::camera::Camera;
pub use self::material::Material;
pub use self::renderer::{Renderer, SuperSampling};
