use math::Vector3;
use color::Color;

pub struct PointLight {
    pub origin: Vector3,
    color: Color,
    intensity: f64,
}

impl PointLight {
    pub fn new(origin: Vector3, color: Color, intensity: f64) -> PointLight {
        PointLight {
            origin: origin,
            color: color,
            intensity: intensity,
        }
    }

    pub fn color(&self) -> Color {
        self.color * self.intensity
    }
}
