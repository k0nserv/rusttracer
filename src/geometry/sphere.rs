use math::{Transform, Point3};
use geometry::{Shape, Intersectable, Transformable};
use intersection::Intersection;
use ray::Ray;
use material::Material;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Point3,
    pub radius: f64,
    material: Material,
}


impl Sphere {
    pub fn new(origin: Point3, radius: f64, material: Material) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            material: material,
        }
    }
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        return &self.material;
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray, _: bool) -> Option<Intersection> {
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
            let point: Point3 = (ray.origin + ray.direction * t.unwrap()).as_point();
            let n = (point - self.origin).normalize();

            let intersection = Intersection::new(t.unwrap(), self, point, ray, n, inside);

            return Some(intersection);
        }


        None
    }
}

impl Transformable for Sphere {
    fn transform(&mut self, transform: &Transform) {
        self.origin = self.origin * transform.matrix;
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use math::{Vector3, Point3, EPSILON};
    use ray::Ray;
    use geometry::Shape;
    use material::{MaterialTemplate, Material};
    use color::Color;

    fn build_test_material() -> Material {
        let color = Color::new(0, 0, 0);

        MaterialTemplate::new(color, color, color, None, None).build_material(|_ignore| {})
    }

    #[test]
    fn test_intersection_miss() {
        let material = build_test_material();
        let sphere = Sphere::new(Point3::at_origin(), 1.0, material);
        let ray = Ray::new(Point3::new(0.0, 0.0, 2.0),
                           Vector3::new(0.0, 0.0, 1.0),
                           None);

        let intersection = (&sphere as &Shape).intersect(ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn test_intersection() {
        let material = build_test_material();
        let sphere = Sphere::new(Point3::at_origin(), 1.0, material);
        let ray = Ray::new(Point3::new(0.0, 0.0, 2.0),
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
