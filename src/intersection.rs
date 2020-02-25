use geometry::Shape;
use math::{Point3, Vector3};
use ray::Ray;
use texture::TextureCoord;

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub shape: &'a dyn Shape,
    pub point: Point3,
    pub ray: Ray,
    pub normal: Vector3,
    pub inside: bool,
    pub texture_coord: Option<TextureCoord>,
}

impl<'a> Intersection<'a> {
    pub fn new(
        t: f32,
        shape: &'a dyn Shape,
        point: Point3,
        ray: Ray,
        normal: Vector3,
        inside: bool,
        texture_coord: Option<TextureCoord>,
    ) -> Intersection {
        Intersection {
            t,
            shape,
            point,
            ray,
            normal,
            inside,
            texture_coord,
        }
    }
}
