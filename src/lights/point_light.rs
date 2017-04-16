use math::Point3;
use color::Color;

pub struct PointLight {
    pub origin: Point3,
    color: Color,
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

    pub fn color(&self) -> Color {
        self.color * self.intensity
    }
}
