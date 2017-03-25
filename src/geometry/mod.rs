pub mod sphere;
pub use self::sphere::Sphere;

use intersection::Intersection;
use ray::Ray;
use material::Material;

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
    fn material(&self) -> &Material;
}
