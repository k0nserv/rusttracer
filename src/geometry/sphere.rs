use math::Vector3;
use geometry::Shape;
use intersection::Intersection;
use ray::Ray;
use material::Material;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vector3,
    pub radius: f64,
    material: Material,
}


impl Sphere {
    pub fn new(origin: Vector3, radius: f64, material: Material) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            material: material,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let v = ray.origin - self.origin;
        let a = ray.direction.dot(&v);
        let b = -a;
        let c = a.powf(2.0) - v.length().powf(2.0) + self.radius.powf(2.0);

        if c < 0.0 {
            return None;
        }

        let t1 = b + c.sqrt();
        let t2 = b - c.sqrt();

        let mut t: Option<f64> = None;
        let mut hit = false;
        let mut inside = false;

        if t1 > 0.01 {
            if t2 < 0.0 {
                t = Some(t1);
                hit = true;
                inside = true;
            } else {
                t = Some(t2);
                hit = true;
            }
        }

        if hit {
            assert!(t.is_some());
            let point = ray.origin + ray.direction * t.unwrap();
            let n = (point - self.origin).normalize();

            let intersection = Intersection::new(t.unwrap(), self, point, ray, n, inside);

            return Some(intersection);
        }


        None
    }

    fn material(&self) -> &Material {
        return &self.material;
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use math::{Vector3, EPSILON};
    use ray::Ray;
    use geometry::Shape;

    #[test]
    fn test_intersection_miss() {
        let sphere = Sphere::new(Vector3::at_origin(), 1.0);
        let ray = Ray::new(Vector3::new(0.0, 0.0, 2.0),
                           Vector3::new(0.0, 0.0, 1.0),
                           None);

        let intersection = (&sphere as &Shape).intersect(ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn test_intersection() {
        let sphere = Sphere::new(Vector3::at_origin(), 1.0);
        let ray = Ray::new(Vector3::new(0.0, 0.0, 2.0),
                           Vector3::new(0.0, 0.0, -1.0),
                           None);

        let i = (&sphere as &Shape).intersect(ray);
        assert!(i.is_some());

        let intersection = i.unwrap();

        assert_eq_within_bound!(intersection.t, 1.0, EPSILON);
        assert_eq_vector3!(intersection.point, Vector3::new(0.0, 0.0, 1.0), EPSILON);
        assert_eq_vector3!(intersection.normal, Vector3::new(0.0, 0.0, 1.0), EPSILON);
    }
}
