use geometry::{Intersectable, Shape, Transformable};
use intersection::Intersection;
use material::Material;
use math::EPSILON;
use math::{Point3, Transform, Vector3};
use ray::Ray;

pub struct Plane {
    pub origin: Point3,
    pub normal: Vector3,
    material: Material,
}

impl Plane {
    pub fn new(origin: Point3, normal: Vector3, material: Material) -> Plane {
        Plane {
            origin,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray, _: bool) -> Option<Intersection> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() <= EPSILON {
            return None;
        }

        let t = ((self.origin - ray.origin).dot(&self.normal)) / denominator;

        if t >= 0.0 {
            let intersection_point = (ray.origin + ray.direction * t).as_point();

            let intersection =
                Intersection::new(t, self, intersection_point, ray, self.normal, false);

            return Some(intersection);
        }

        None
    }
}

impl Transformable for Plane {
    fn transform(&mut self, transform: &Transform) {
        self.origin = self.origin * transform.matrix;
        self.normal = (self.normal * transform.normal_matrix).normalize();
    }
}
