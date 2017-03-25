use math::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub medium_refraction: f64,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3, medium_refraction: Option<f64>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            medium_refraction: medium_refraction.unwrap_or(1.0),
        }
    }
}
