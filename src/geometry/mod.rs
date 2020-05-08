mod aabb;
mod extent_volume;
mod mesh;
mod plane;
mod simple_triangle_storage;
mod sphere;
pub mod triangle;

pub use self::aabb::AABB;
pub use self::extent_volume::ExtentVolume;
pub use self::mesh::Mesh;
pub use self::plane::Plane;
pub use self::simple_triangle_storage::SimpleTriangleStorage;
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
    fn new(triangles: &mut dyn Iterator<Item = &Triangle>) -> Self;
    fn intersect(&self, ray: Ray) -> bool;
}

pub trait TriangleStorage<'a> {
    type Iterator: Iterator<Item = &'a Triangle>;
    type IteratorMut: Iterator<Item = &'a mut Triangle>;

    fn new(triangles: Vec<Triangle>) -> Self;
    fn intersect(&'a self, ray: Ray, cull: bool) -> Self::Iterator;
    fn all(&'a self) -> Self::Iterator;
    fn all_mut(&'a mut self) -> Self::IteratorMut;
}

impl<T> Transformable for T
where
    T: for<'a> TriangleStorage<'a>,
{
    fn transform(&mut self, transform: &Transform) {
        for triangle in self.all_mut() {
            triangle.transform(&transform);
        }
    }
}
