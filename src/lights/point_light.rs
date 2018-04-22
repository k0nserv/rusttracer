use color::Color;
use math::Point3;

pub struct PointLight {
    pub origin: Point3,
    pub color: Color,
    intensity: f32,
}

impl PointLight {
    pub fn new(origin: Point3, color: Color, intensity: f32) -> PointLight {
        PointLight {
            origin,
            color,
            intensity,
        }
    }

    pub fn intensity(&self, distance_to_light: f32) -> f32 {
        1.0 / (distance_to_light * distance_to_light) * self.intensity
    }
}
