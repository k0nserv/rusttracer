use std::f32::consts::PI;
use std::rc::Rc;

use geometry::{Intersectable, Shape, Transformable};
use intersection::Intersection;
use material::Material;
use math::{Point3, Transform};
use ray::Ray;
use texture::TextureCoord;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Point3,
    pub radius: f32,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(origin: Point3, radius: f32, material: Rc<Material>) -> Self {
        Self {
            origin,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
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

        let (t, hit, inside) = if t1 > 0.01 {
            if t2 < 0.0 {
                (Some(t1), true, true)
            } else {
                (Some(t2), true, false)
            }
        } else {
            (None, false, false)
        };

        if hit {
            assert!(t.is_some());
            let point: Point3 = (ray.origin + ray.direction * t.unwrap()).as_point();
            let normal = (point - self.origin).normalize();
            let texture_coord = TextureCoord::new(
                normal.x.atan2(normal.z) / (2.0 * PI) + 0.5,
                normal.y * 0.5 + 0.5,
            );

            let intersection = Intersection::new(
                t.unwrap(),
                self,
                point,
                ray,
                normal,
                inside,
                Some(texture_coord),
            );

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
    use std::rc::Rc;

    use super::Sphere;
    use color::Color;
    use geometry::Shape;
    use material::{IllumninationModel, Material, MaterialTemplate};
    use math::{Point3, Vector3, EPSILON};
    use ray::Ray;

    fn build_test_material() -> Rc<Material> {
        let color = Color::new(0, 0, 0);

        Rc::new(
            MaterialTemplate::new(
                color,
                color,
                color,
                0.0,
                IllumninationModel::Constant,
                None,
                None,
            ).build_material(|_ignore| {}),
        )
    }

    #[test]
    fn test_intersection_miss() {
        let material = build_test_material();
        let sphere = Sphere::new(Point3::at_origin(), 1.0, material);
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 2.0),
            Vector3::new(0.0, 0.0, 1.0),
            None,
        );

        let intersection = (&sphere as &Shape).intersect(ray, false);

        assert!(intersection.is_none());
    }

    #[test]
    fn test_intersection() {
        let material = build_test_material();
        let sphere = Sphere::new(Point3::at_origin(), 1.0, material);
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 2.0),
            Vector3::new(0.0, 0.0, -1.0),
            None,
        );

        let i = (&sphere as &Shape).intersect(ray, false);
        assert!(i.is_some());

        let intersection = i.unwrap();

        assert_eq_within_bound!(intersection.t, 1.0, EPSILON);
        assert_eq_vector3!(intersection.point, Vector3::new(0.0, 0.0, 1.0), EPSILON);
        assert_eq_vector3!(intersection.normal, Vector3::new(0.0, 0.0, 1.0), EPSILON);
    }
}
