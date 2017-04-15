pub mod plane;
pub mod sphere;

pub use self::plane::Plane;
pub use self::sphere::Sphere;

use intersection::Intersection;
use ray::Ray;
use material::Material;

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

pub trait Shape: Intersectable {
    fn material(&self) -> &Material;
}
