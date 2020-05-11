use super::triangle;
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

    pub fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        let bounding_box = Self::from_triangle(triangle);

        let min = self.min();
        let max = self.max();

        let b_min = bounding_box.min();
        let b_max = bounding_box.max();

        return min.x <= b_max.x
            && max.x >= b_min.x
            && min.y <= b_max.y
            && max.y >= b_min.y
            && min.z <= b_max.z
            && max.z >= b_min.z;

        // TODO: Finish this and make it work based on
        // http://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/code/tribox_tam.pdf
        let center = self.max() - self.min();
        let half = center * 0.5;

        let v0 = triangle.vertices[0].as_vector() - center;
        let v1 = triangle.vertices[1].as_vector() - center;
        let v2 = triangle.vertices[2].as_vector() - center;

        // Triangle vectors
        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        // 0, 0
        {
            let p0 = f0.y * v0.z - f0.z * v0.y;
            let p2 = f0.y * v2.z - f0.z * v2.y;
            let r = half.y * f0.y.abs() + half.z * f0.z.abs();

            if f32::min(p0, p2) > r || f32::max(p0, p2) < -r {
                return false;
            }
        }

        // 0, 1
        {
            let p0 = f1.y * v0.z - f1.z * v0.y;
            let p1 = f1.y * v1.z - f1.z * v1.y;
            let r = half.y * f1.y.abs() + half.z * f1.z.abs();

            if f32::min(p0, p1) > r || f32::max(p0, p1) < -r {
                return false;
            }
        }

        // 0, 2
        {
            let p0 = f2.x * v0.y - f2.y * v0.x;
            let p1 = f2.x * v1.y - f2.y * v1.x;
            let r = half.x * f2.x.abs() + half.y * f2.y.abs();

            if f32::min(p0, p1) > r || f32::max(p0, p1) < -r {
                return false;
            }
        }

        // 1, 0
        {
            let p0 = f0.z * v0.x - f0.x * v0.z;
            let p2 = f0.z * v2.x - f0.x * v2.z;
            let r = half.x * f0.x.abs() + half.z * f0.z.abs();

            if f32::min(p0, p2) > r || f32::max(p0, p2) < -r {
                return false;
            }
        }

        // 1, 1
        {
            let p0 = f1.z * v0.x - f1.x * v0.x;
            let p1 = f1.z * v1.x - f1.x * v1.x;
            let r = half.x * f1.x.abs() + half.z * f1.z.abs();

            if f32::min(p0, p1) > r || f32::max(p0, p1) < -r {
                return false;
            }
        }

        // 1, 2
        {
            let p0 = f2.z * v0.x - f2.x * v0.z;
            let p1 = f2.z * v1.x - f2.x * v1.z;
            let r = half.x * f2.x.abs() + half.z * f2.z.abs();

            if f32::min(p0, p1) > r || f32::max(p0, p1) < -r {
                return false;
            }
        }

        // 2, 0
        {
            let p0 = f0.x * v0.y - f0.y * v0.x;
            let p2 = f0.x * v2.y - f0.y * v2.x;
            let r = half.x * f0.x.abs() + half.y * f0.y.abs();

            if f32::min(p0, p2) > r || f32::max(p0, p2) < -r {
                return false;
            }
        }

        // 1, 2
        {
            let p0 = f1.z * v0.x - f1.x * v0.z;
            let p1 = f1.z * v1.x - f1.x * v1.z;
            let r = half.x * f1.x.abs() + half.z * f1.z.abs();

            if f32::min(p0, p1) > r || f32::max(p0, p1) < -r {
                return false;
            }
        }

        // 2, 2
        {
            let p0 = f2.x * v0.y - f2.y * v0.x;
            let p1 = f2.x * v1.y - f2.y * v1.x;
            let r = half.x * f2.x.abs() + half.z * f2.z.abs();

            if f32::min(p0, p1) > r || f32::max(p0, p1) < -r {
                return false;
            }
        }

        let max_x = v0.x.max(v1.x.max(v2.x));
        let min_x = v0.x.min(v1.x.min(v2.x));

        if min_x > half.x || max_x < -half.x {
            return false;
        }

        let max_y = v0.y.max(v1.y.max(v2.y));
        let min_y = v0.y.min(v1.y.min(v2.y));

        if min_y > half.y || max_y < -half.y {
            return false;
        }

        let max_z = v0.z.max(v1.z.max(v2.z));
        let min_z = v0.z.min(v1.z.min(v2.z));

        if min_z > half.z || max_z < -half.z {
            return false;
        }

        match triangle.normal {
            triangle::Normal::Face(normal) => {
                if !Self::plane_box_overlap(normal, v0, half) {
                    return false;
                }
            }
            triangle::Normal::Vertex(n0, n1, n2) => {
                if !Self::plane_box_overlap(n0, v0, half)
                    && !Self::plane_box_overlap(n1, v1, half)
                    && !Self::plane_box_overlap(n2, v2, half)
                {
                    return false;
                }
            }
        }

        // // Box  normals
        // let u0 = Vector3::new(1.0, 0.0, 0.0);
        // let u1 = Vector3::new(0.0, 1.0, 0.0);
        // let u2 = Vector3::new(0.0, 0.0, 1.0);

        // // 9 Axis separating triangle and box
        // let axis_u0_f0 = u0.cross(&f0);
        // let axis_u0_f1 = u0.cross(&f1);
        // let axis_u0_f2 = u0.cross(&f2);

        // let axis_u1_f0 = u1.cross(&f0);
        // let axis_u1_f1 = u1.cross(&f1);
        // let axis_u1_f2 = u1.cross(&f2);

        // let axis_u2_f0 = u2.cross(&f0);
        // let axis_u2_f1 = u2.cross(&f1);
        // let axis_u2_f2 = u2.cross(&f2);

        // // Edge 0
        // if Self::separted_on_triangle_edge_0(v0, v2, axis_u0_f0, half) {
        //     return true;
        // }

        // if Self::separted_on_triangle_edge_0(v0, v2, axis_u0_f1, half) {
        //     return true;
        // }

        // if Self::separted_on_triangle_edge_0(v0, v2, axis_u0_f2, half) {
        //     return true;
        // }

        // // Edge 1
        // if Self::separted_on_triangle_edge_1(v0, v1, axis_u1_f0, half) {
        //     return true;
        // }

        // if Self::separted_on_triangle_edge_1(v0, v1, axis_u1_f1, half) {
        //     return true;
        // }

        // if Self::separted_on_triangle_edge_1(v0, v1, axis_u1_f2, half) {
        //     return true;
        // }

        // // Edge 2
        // if Self::separted_on_triangle_edge_2(v1, v2, axis_u2_f0, half) {
        //     return true;
        // }

        // if Self::separted_on_triangle_edge_2(v1, v2, axis_u2_f1, half) {
        //     return true;
        // }

        // if Self::separted_on_triangle_edge_2(v1, v2, axis_u2_f2, half) {
        //     return true;
        // }
        true

        // for vertex in &triangle.vertices {
        //     if vertex.x <= self.max().x
        //         && vertex.x >= self.min().x
        //         && vertex.y <= self.max().y
        //         && vertex.y >= self.min().y
        //         && vertex.z <= self.max().z
        //         && vertex.z >= self.min().z
        //     {
        //         return true;
        //     }
        // }

        // false
    }

    fn plane_box_overlap(normal: Vector3, vertex: Vector3, half: Vector3) -> bool {
        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        if normal.x > 0.0 {
            min.x = -half.x - vertex.x;
            max.x = half.x - vertex.x;
        } else {
            min.x = half.x - vertex.x;
            max.x = -half.x - vertex.x;
        }

        if normal.y > 0.0 {
            min.y = -half.y - vertex.y;
            max.y = half.y - vertex.y;
        } else {
            min.y = half.y - vertex.y;
            max.y = -half.y - vertex.y;
        }

        if normal.z > 0.0 {
            min.z = -half.z - vertex.z;
            max.z = half.z - vertex.z;
        } else {
            min.z = half.z - vertex.z;
            max.z = -half.z - vertex.z;
        }

        if normal.dot(&min.as_vector()) > 0.0 {
            return false;
        }

        if normal.dot(&max.as_vector()) >= 0.0 {
            return true;
        }

        false
    }

    fn axis_test0(v0: Vector3, v2: Vector3, axis: Vector3, half: Vector3) -> bool {
        let p0 = v0.dot(&axis);
        let p2 = v2.dot(&axis);

        let r = half.x * &axis.x.abs() + half.y * &axis.y.abs() + half.z * &axis.z.abs();

        f32::min(p0, p2) > r || f32::max(p0, p2) < -r
    }

    fn separted_on_triangle_edge_1(v0: Vector3, v1: Vector3, axis: Vector3, half: Vector3) -> bool {
        let p0 = v0.dot(&axis);
        let p1 = v1.dot(&axis);

        let r = half.x * &axis.x.abs() + half.y * &axis.y.abs() + half.z * &axis.z.abs();

        f32::min(p0, p1) > r || f32::max(p0, p1) < -r
    }

    fn separted_on_triangle_edge_2(v1: Vector3, v2: Vector3, axis: Vector3, half: Vector3) -> bool {
        let p1 = v1.dot(&axis);
        let p2 = v2.dot(&axis);

        let r = half.x * &axis.x.abs() + half.y * &axis.y.abs() + half.z * &axis.z.abs();

        f32::min(p1, p2) > r || f32::max(p1, p2) < -r
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
