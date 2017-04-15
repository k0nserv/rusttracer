use math::Vector3;
use geometry::{Shape, Intersectable};
use material::Material;
use intersection::Intersection;
use ray::Ray;
use math::EPSILON;

pub struct Plane {
    pub origin: Vector3,
    pub normal: Vector3,
    material: Material,
}

impl Plane {
    pub fn new(origin: Vector3, normal: Vector3, material: Material) -> Plane {
        Plane {
            origin: origin,
            normal: normal.normalize(),
            material: material,
        }
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() <= EPSILON {
            return None;
        }

        let t = ((self.origin - ray.origin).dot(&self.normal)) / denominator;

        if t >= 0.0 {
            let intersection_point = ray.origin + ray.direction * t;

            let intersection =
                Intersection::new(t, self, intersection_point, ray, self.normal, false);

            return Some(intersection);
        }

        None
    }
}
