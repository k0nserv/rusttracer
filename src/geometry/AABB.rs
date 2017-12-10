use math::Point3;
use ray::Ray;
use super::{BoundingVolume, Triangle};

#[derive(Debug)]
pub struct AABB {
    bounds: [Point3; 2],
}

impl BoundingVolume for AABB {
    fn new(triangles: &[Box<Triangle>]) -> Self {
        assert!(triangles.len() > 0, "Creating AABB with 0 vertices");
        if triangles.len() == 0 {
            return AABB {
                bounds: [Point3::at_origin(), Point3::at_origin()],
            };
        }
        let mut min = triangles[0].vertices[0];
        let mut max = triangles[0].vertices[0];

        for triangle in triangles.iter() {
            for vertex in triangle.vertices.iter() {
                // Max
                if vertex.x > max.x {
                    max.x = vertex.x;
                }

                if vertex.y > max.y {
                    max.y = vertex.y;
                }

                if vertex.z > max.z {
                    max.z = vertex.z;
                }

                // Min
                if vertex.x < min.x {
                    min.x = vertex.x;
                }

                if vertex.y < min.y {
                    min.y = vertex.y;
                }

                if vertex.z < min.z {
                    min.z = vertex.z;
                }
            }
        }

        AABB { bounds: [min, max] }
    }

    fn intersect(&self, ray: Ray) -> bool {
        let mut tmin = (self.bounds[ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;
        let mut tmax = (self.bounds[1 - ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;

        let tymin = (self.bounds[ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;
        let tymax = (self.bounds[1 - ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds[ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;
        let tzmax = (self.bounds[1 - ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        let mut t = tmin;

        if t < 0.0 {
            t = tmax;

            if t < 0.0 {
                return false;
            }
        }


        true
    }
}
