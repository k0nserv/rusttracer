use math::Point3;
use color::Color;

pub struct PointLight {
    pub origin: Point3,
    pub color: Color,
    intensity: f64,
}

impl PointLight {
    pub fn new(origin: Point3, color: Color, intensity: f64) -> PointLight {
        PointLight {
            origin: origin,
            color: color,
            intensity: intensity,
        }
    }

    pub fn intensity(&self, distance_to_light: f64) -> f64 {
        1.0 / (distance_to_light * distance_to_light) * self.intensity
    }
}
