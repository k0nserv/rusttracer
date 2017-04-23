pub mod plane;
pub mod sphere;
pub mod triangle;
pub mod mesh;

pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;
pub use self::mesh::Mesh;

use intersection::Intersection;
use ray::Ray;
use math::Transform;
use material::Material;

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

pub trait Transformable {
    fn transform(&mut self, transform: &Transform);
}

pub trait Shape: Intersectable + Transformable {
    fn material(&self) -> &Material;
}
