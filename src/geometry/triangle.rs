use math::{Vector3, Transform, Point3};
use geometry::{Shape, Intersectable, Transformable};
use ray::Ray;
use material::Material;
use intersection::Intersection;
use math::EPSILON;

#[derive(Debug)]
pub struct Triangle {
    pub a: Point3, // A
    pub b: Point3, // B
    pub c: Point3, // C
    pub ab: Vector3, // B - A
    pub ac: Vector3, // C - A
    pub normal: Vector3,
    material: Material,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, material: Material) -> Triangle {
        let ab = b - a;
        let ac = c - a;

        Triangle {
            a: a,
            b: b,
            c: c,
            ab: ab,
            ac: ac,
            normal: ab.cross(&ac).normalize(),
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
        let pvec = ray.direction.cross(&self.ac);
        let det = self.ab.dot(&pvec);

        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.a;
        let u = tvec.dot(&pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&self.ab);
        let v = ray.direction.dot(&qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = self.ac.dot(&qvec) * inv_det;
        if t > EPSILON {

            let intersection_point = (ray.origin + ray.direction * t).as_point();
            let intersection =
                Intersection::new(t, self, intersection_point, ray, self.normal, false);

            return Some(intersection);
        }

        None
    }
}

impl Transformable for Triangle {
    fn transform(&mut self, transform: &Transform) {
        let matrix = transform.matrix;
        let normal_matrix = transform.matrix;

        self.a = self.a * matrix;
        self.b = self.b * matrix;
        self.c = self.c * matrix;
        self.ab = self.b - self.a;
        self.ac = self.c - self.a;
        self.normal = (self.normal * normal_matrix).normalize();
    }
}
