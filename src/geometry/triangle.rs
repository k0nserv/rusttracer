use math::{Vector3, Transform, Point3};
use geometry::{Shape, Intersectable, Transformable};
use ray::Ray;
use material::Material;
use intersection::Intersection;
use math::EPSILON;

#[derive(Debug)]
pub struct Triangle {
    // A, B, C
    pub vertices: [Point3; 3],
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
            vertices: [a, b, c],
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
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection> {
        let pvec = ray.direction.cross(&self.ac);
        let det = self.ab.dot(&pvec);

        if cull && det < EPSILON {
            return None;
        }

        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.vertices[0];
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

        // TODO: Consider doing this as a 4x4 matrix calculation instead
        self.vertices[0] = self.vertices[0] * matrix;
        self.vertices[1] = self.vertices[1] * matrix;
        self.vertices[2] = self.vertices[2] * matrix;
        self.ab = self.vertices[1] - self.vertices[0];
        self.ac = self.vertices[2] - self.vertices[0];
        self.normal = (self.normal * normal_matrix).normalize();
    }
}
