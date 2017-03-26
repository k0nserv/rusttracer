use math::Vector3;
use geometry::Shape;
use material::Material;
use intersection::Intersection;
use ray::Ray;

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
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() < 1e-5 {
            return None;
        }

        let t = (self.origin - ray.origin).dot(&self.normal) / denominator;

        if t >= 1e-5 {
            let intersectionPoint = ray.origin + ray.direction * t;

            let intersection =
                Intersection::new(t, self, intersectionPoint, ray, self.normal, false);

            return Some(intersection);
        }

        None
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
