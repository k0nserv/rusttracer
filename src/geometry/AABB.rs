use math::Point3;
use ray::Ray;

#[derive(Debug)]
pub struct AABB {
    bounds: [Point3; 2],
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { bounds: [min, max] }
    }

    pub fn intersect(&self, ray: Ray) -> bool {
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
