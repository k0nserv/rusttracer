mod aabb;
mod mesh;
mod plane;
mod sphere;
pub mod triangle;

pub use self::aabb::AABB;
pub use self::mesh::Mesh;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

use intersection::Intersection;
use material::Material;
use math::Transform;
use ray::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection>;
}

pub trait Transformable {
    fn transform(&mut self, transform: &Transform);
}

pub trait Shape: Intersectable {
    fn material(&self) -> &Material;
}

pub trait BoundingVolume {
    fn new(triangles: &[Triangle]) -> Self;
    fn intersect(&self, ray: Ray) -> bool;
}
