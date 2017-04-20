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
    pub ab: Vector3, // B - A
    pub ac: Vector3, // C - A
    pub normal: Vector3,
    material: Material,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Material) -> Triangle {
        let ab = v1 - v0;
        let ac = v2 - v0;

        Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
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

        let tvec = ray.origin - self.v0;
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
    fn transform(&mut self, matrix: Matrix4, normal_matrix: Matrix4) {
        self.v0 = self.v0 * matrix;
        self.v1 = self.v1 * matrix;
        self.v2 = self.v2 * matrix;
        self.normal = (self.normal * normal_matrix).normalize();
    }
}
