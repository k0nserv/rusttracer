use super::{BoundingVolume, Triangle};
use math::{Point3, Vector3};
use ray::Ray;

#[derive(Debug, Clone)]
pub struct AABB {
    bounds: [Point3; 2],
}

impl Default for AABB {
    fn default() -> Self {
        Self::empty()
    }
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        assert!(min.z <= max.z);

        Self { bounds: [min, max] }
    }

    pub fn from_triangle(triangle: &Triangle) -> Self {
        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for vertex in &triangle.vertices {
            // Max
            max.x = f32::max(vertex.x, max.x);
            max.y = f32::max(vertex.y, max.y);
            max.z = f32::max(vertex.z, max.z);

            // Min
            min.x = f32::min(vertex.x, min.x);
            min.y = f32::min(vertex.y, min.y);
            min.z = f32::min(vertex.z, min.z);
        }

        Self::new(min, max)
    }

    pub fn empty() -> Self {
        Self {
            bounds: [
                Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
                Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            ],
        }
    }

    pub fn dimensions(&self) -> Vector3 {
        self.max() - self.min()
    }

    pub fn min(&self) -> Point3 {
        self.bounds[0]
    }

    pub fn max(&self) -> Point3 {
        self.bounds[1]
    }

    pub fn half(&self) -> Point3 {
        ((self.max() - self.min()) * 0.5).as_point()
    }

    pub fn center(&self) -> Point3 {
        (self.min() + self.half().as_vector()).as_point()
    }

    pub fn intersects_triangle_aabb(&self, triangle: &Triangle) -> bool {
        let bounding_box = Self::from_triangle(triangle);

        let min = self.min();
        let max = self.max();

        let b_min = bounding_box.min();
        let b_max = bounding_box.max();

        min.x <= b_max.x
            && max.x >= b_min.x
            && min.y <= b_max.y
            && max.y >= b_min.y
            && min.z <= b_max.z
            && max.z >= b_min.z
    }
}

impl BoundingVolume for AABB {
    fn from_triangles(triangles: &mut dyn Iterator<Item = &Triangle>) -> Self {
        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for triangle in triangles {
            for vertex in &triangle.vertices {
                // Max
                max.x = f32::max(vertex.x, max.x);
                max.y = f32::max(vertex.y, max.y);
                max.z = f32::max(vertex.z, max.z);

                // Min
                min.x = f32::min(vertex.x, min.x);
                min.y = f32::min(vertex.y, min.y);
                min.z = f32::min(vertex.z, min.z);
            }
        }

        Self::new(min, max)
    }

    fn intersect(&self, ray: Ray) -> bool {
        let mut tmin = (self.bounds[ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;
        let mut tmax = (self.bounds[1 - ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;

        let tymin = (self.bounds[ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;
        let tymax = (self.bounds[1 - ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        tmin = f32::max(tymin, tmin);
        tmax = f32::min(tymax, tmax);

        let tzmin = (self.bounds[ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;
        let tzmax = (self.bounds[1 - ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        tmin = f32::max(tzmin, tmin);
        tmax = f32::min(tzmax, tmax);

        let mut t = tmin;

        if t <= 0.0 {
            t = tmax;

            if t <= 0.0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::AABB;
    use color::Color;
    use geometry::triangle::{Normal, Triangle};
    use material::{IllumninationModel, Material};
    use math::{Point3, Vector3};

    fn make_material() -> Material {
        Material::new(
            Color::black(),
            Color::black(),
            Color::black(),
            0.0,
            IllumninationModel::Constant,
            None,
            None,
        )
    }

    #[test]
    fn test_intersects_triangle_aabb_vertex_inside() {
        let aabb = AABB::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(2.0, 2.0, 2.0));

        let triangle = Triangle::new(
            Point3::at_origin(),
            Point3::new(3.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 3.0),
            Normal::Face(Vector3::new(0.0, 1.0, 0.0)),
            None,
            Rc::new(make_material()),
        );

        assert!(aabb.intersects_triangle_aabb(&triangle));
    }

    #[test]
    fn test_intersects_triangle_aabb_edge() {
        let aabb = AABB::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(2.0, 2.0, 2.0));

        let triangle = Triangle::new(
            Point3::new(3.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 3.0),
            Point3::new(3.0, 0.0, 3.0),
            Normal::Face(Vector3::new(0.0, 1.0, 0.0)),
            None,
            Rc::new(make_material()),
        );

        assert!(aabb.intersects_triangle_aabb(&triangle));
    }

    #[test]
    fn test_intersects_triangle_aabb_realistic_trivial() {
        // This case is entirely trivial: The triangle beeing tested is tested
        // against the bounding box of the whole mesh.

        let aabb = AABB::new(
            Point3::new(-0.23978, -0.282958, -0.472247),
            Point3::new(0.207395, 0.422022, 0.527753),
        );

        let triangle = Triangle::new(
            Point3::new(-0.0148929, -0.270744, 0.213293),
            Point3::new(-0.0132528, -0.270767, 0.213397),
            Point3::new(-0.0146446, -0.270253, 0.214432),
            Normal::Vertex(
                Vector3::new(-0.0331532, -0.915051, 0.401972),
                Vector3::new(-0.056424, -0.938723, 0.340022),
                Vector3::new(-0.114637, -0.897883, 0.425047),
            ),
            None,
            Rc::new(make_material()),
        );

        assert!(aabb.intersects_triangle_aabb(&triangle));
    }

    #[test]
    fn test_intersects_triangle_aabb_realistic_1() {
        let aabb = AABB::new(
            Point3::new(-0.23978, -0.282958, -0.472247),
            Point3::new(0.207395, 0.422022, 0.527753),
        );

        let triangle = Triangle::new(
            Point3::new(-0.219407, -0.248815, -0.229673),
            Point3::new(-0.218334, -0.252232, -0.224549),
            Point3::new(-0.219816, -0.248815, -0.219424),
            Normal::Vertex(
                Vector3::new(-0.93934, -0.338087, -0.0577654),
                Vector3::new(-0.935684, -0.351144, -0.0345402),
                Vector3::new(-0.943101, -0.331952, -0.019191),
            ),
            None,
            Rc::new(make_material()),
        );

        assert!(aabb.intersects_triangle_aabb(&triangle));
    }

    #[test]
    fn test_intersects_triangle_aabb_realistic_2() {
        let aabb = AABB::new(
            Point3::new(-0.23978, -0.282958, -0.472247),
            Point3::new(0.207395, 0.422022, 0.527753),
        );

        let triangle = Triangle::new(
            Point3::new(0.136228, 0.217532, -0.149386),
            Point3::new(0.135274, 0.217532, -0.150971),
            Point3::new(0.136983, 0.21924, -0.149388),
            Normal::Vertex(
                Vector3::new(0.85279, -0.201127, -0.481972),
                Vector3::new(0.820284, -0.230435, -0.523482),
                Vector3::new(0.732158, -0.430477, -0.527859),
            ),
            None,
            Rc::new(make_material()),
        );

        assert!(aabb.intersects_triangle_aabb(&triangle));
    }
}
