use geometry::Shape;
use color::Color;
use ray::Ray;
use intersection::Intersection;
use lights::PointLight;

pub struct Scene<'a> {
    pub objects: &'a Vec<&'a Shape>,
    pub lights: &'a Vec<&'a PointLight>,
    pub clear_color: Color,
}

impl<'a> Scene<'a> {
    pub fn new(objects: &'a Vec<&'a Shape>,
               lights: &'a Vec<&'a PointLight>,
               clear_color: Color)
               -> Scene<'a> {
        Scene {
            objects: objects,
            lights: lights,
            clear_color: clear_color,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;

        for shape in self.objects {
            if let Some(intersection) = shape.intersect(ray) {
                if let Some(closest) = closest_intersection {
                    if intersection.t < closest.t {
                        closest_intersection = Some(intersection)
                    }
                } else {
                    closest_intersection = Some(intersection)
                }
            }
        }

        return closest_intersection;
    }
}
