use math::{Vector3, Matrix4, Point3};
use geometry::{Shape, Intersectable, Transformable};
use ray::Ray;
use material::Material;
use intersection::Intersection;
use math::EPSILON;

#[derive(Debug)]
pub struct Triangle {
    pub v0: Point3, // A
    pub v1: Point3, // B
    pub v2: Point3, // C
    pub d: f64, // Distance to origin
    pub normal: Vector3,
    material: Material,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Material) -> Triangle {
        let a = v1 - v0;
        let b = v2 - v0;
        let normal = a.cross(&b).normalize();

        Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
            d: normal.dot(&v0.as_vector()).abs(),
            normal: normal.normalize(),
            material: material,
        }
    }
}

impl Shape for Triangle {
    fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let normal_dot_ray_dir = self.normal.dot(&ray.direction);

        // Perpendicular to triangle plane
        if normal_dot_ray_dir.abs() <= EPSILON {
            return None;
        }

        let t = -(self.normal.dot(&ray.origin.as_vector()) + self.d) / normal_dot_ray_dir;

        if t < 0.0 {
            return None;
        }

        let intersection_point = (ray.origin + ray.direction * t).as_point();

        let edge0 = self.v1 - self.v0;
        let vp0 = intersection_point - self.v0;
        let mut c = edge0.cross(&vp0);

        if self.normal.dot(&c) < 0.0 {
            return None;
        }

        let edge1 = self.v2 - self.v1;
        let vp1 = intersection_point - self.v1;
        c = edge1.cross(&vp1);

        if self.normal.dot(&c) < 0.0 {
            return None;
        }

        let edge2 = self.v0 - self.v2;
        let vp2 = intersection_point - self.v2;
        c = edge2.cross(&vp2);

        if self.normal.dot(&c) < 0.0 {
            return None;
        }

        let intersection = Intersection::new(t, self, intersection_point, ray, self.normal, false);

        Some(intersection)
    }
}

impl Transformable for Triangle {
    fn transform(&mut self, matrix: Matrix4, normal_matrix: Matrix4) {
        self.v0 = self.v0 * matrix;
        self.v1 = self.v1 * matrix;
        self.v2 = self.v2 * matrix;
        self.normal = (self.normal * normal_matrix).normalize();
        self.d = self.normal.dot(&self.v0.as_vector()).abs();
    }
}
