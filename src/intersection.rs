use math::{Point3, Vector3};
use ray::Ray;
use geometry::Shape;

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub shape: &'a Shape,
    pub point: Point3,
    pub ray: Ray,
    pub normal: Vector3,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64,
               shape: &'a Shape,
               point: Point3,
               ray: Ray,
               normal: Vector3,
               inside: bool)
               -> Intersection {
        Intersection {
            t: t,
            shape: shape,
            point: point,
            ray: ray,
            normal: normal,
            inside: inside,
        }
    }
}
