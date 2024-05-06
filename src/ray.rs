use crate::math::{Point3, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub inv_direction: Vector3,
    pub sign: [usize; 3],
    pub medium_refraction: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3, medium_refraction: Option<f32>) -> Ray {
        let inv_dir = Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        Ray {
            origin,
            direction,
            inv_direction: inv_dir,
            sign: [
                (inv_dir.x < 0.0) as usize,
                (inv_dir.y < 0.0) as usize,
                (inv_dir.z < 0.0) as usize,
            ],
            medium_refraction: medium_refraction.unwrap_or(1.0),
        }
    }
}
