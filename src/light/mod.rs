use color::Color;
use intersection::Intersection;
use material::Material;
use ray::Ray;

pub mod directional;
pub mod point;

pub use self::directional::Directional;
pub use self::point::Point;

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Falloff {
    // Light falls off with the inverse square of the distance
    InverseSquare,
    // Light falls off linearly with the distance
    InverseLinear,
}

pub trait Light {
    fn create_shadow_ray(&self, intersection: &Intersection, medium_refraction: Option<f32>)
        -> Ray;
    fn distance_to_light(&self, intersection: &Intersection) -> f32;
    fn diffuse_color(
        &self,
        intersection: &Intersection,
        material: &Material,
        distance_to_light: f32,
    ) -> Option<Color>;
    fn specular_color(
        &self,
        intersection: &Intersection,
        material: &Material,
        ray: &Ray,
        distance_to_light: f32,
    ) -> Option<Color>;
}
