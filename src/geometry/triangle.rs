use geometry::{Intersectable, Shape, Transformable};
use intersection::Intersection;
use material::Material;
use math::EPSILON;
use math::{Point3, Transform, Vector3};
use ray::Ray;

#[cfg(feature = "stats")]
pub mod stats {
    use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

    static TRIANGLE_INTERSECTION_TEST_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
    static TRIANGLE_INTERSECTION_HIT_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

    pub fn record_triangle_intersection() {
        TRIANGLE_INTERSECTION_TEST_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_triangle_hit() {
        TRIANGLE_INTERSECTION_HIT_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    pub fn number_of_triangle_intersections() -> usize {
        TRIANGLE_INTERSECTION_TEST_COUNT.load(Ordering::SeqCst)
    }

    pub fn number_of_triangle_hits() -> usize {
        TRIANGLE_INTERSECTION_HIT_COUNT.load(Ordering::SeqCst)
    }
}

#[cfg(feature = "stats")]
fn record_triangle_intersection() {
    stats::record_triangle_intersection();
}

#[cfg(not(feature = "stats"))]
fn record_triangle_intersection() {}

#[cfg(feature = "stats")]
fn record_triangle_hit() {
    stats::record_triangle_hit();
}

#[cfg(not(feature = "stats"))]
fn record_triangle_hit() {}

#[derive(Debug)]
pub enum Normal {
    Face(Vector3),
    Vertex(Vector3, Vector3, Vector3),
}

#[derive(Debug)]
pub struct Triangle {
    // A, B, C
    pub vertices: [Point3; 3],
    pub ab: Vector3, // B - A
    pub ac: Vector3, // C - A
    pub normal: Normal,
    material: Material,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, normal: Normal, material: Material) -> Triangle {
        let ab = b - a;
        let ac = c - a;

        Triangle {
            vertices: [a, b, c],
            ab,
            ac,
            normal,
            material,
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
        record_triangle_intersection();

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
            let intersection = Intersection::new(
                t,
                self,
                intersection_point,
                ray,
                self.normal_at_intersection(u, v),
                false,
            );

            record_triangle_hit();
            return Some(intersection);
        }

        None
    }
}

impl Triangle {
    fn normal_at_intersection(&self, u: f32, v: f32) -> Vector3 {
        match self.normal {
            Normal::Face(normal) => normal,
            Normal::Vertex(n0, n1, n2) => (n0 * (1.0 - u - v) + n1 * u + n2 * v).normalize(),
        }
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
        self.normal = match self.normal {
            Normal::Face(normal) => Normal::Face((normal * normal_matrix).normalize()),
            Normal::Vertex(n0, n1, n2) => Normal::Vertex(
                (n0 * normal_matrix).normalize(),
                (n1 * normal_matrix).normalize(),
                (n2 * normal_matrix).normalize(),
            ),
        }
    }
}
