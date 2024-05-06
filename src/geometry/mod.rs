mod aabb;
mod extent_volume;
mod instance;
mod mesh;
mod octtree;
mod plane;
mod simple_triangle_storage;
mod sphere;
pub mod triangle;

pub use self::aabb::AABB;
pub use self::extent_volume::ExtentVolume;
pub use self::instance::Instance;
pub use self::mesh::Mesh;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

pub use self::octtree::Octree;
pub use self::simple_triangle_storage::SimpleTriangleStorage;

use crate::intersection::Intersection;
use crate::material::Material;
use crate::math::Transform;
use crate::ray::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection>;
}

pub trait Transformable {
    fn transform(&mut self, transform: &Transform);
    fn apply_transforms(&mut self, transforms: &[Transform]) {
        for transform in transforms {
            self.transform(transform);
        }
    }
}

pub trait Shape: Intersectable {
    fn material(&self) -> &Material;
}

pub trait BoundingVolume {
    fn from_triangles(triangles: &mut dyn Iterator<Item = &Triangle>) -> Self;
    fn intersect(&self, ray: Ray) -> bool;
}

pub trait TriangleStorage<'a>: Transformable {
    type Iterator: Iterator<Item = &'a Triangle>;
    type IteratorMut: Iterator<Item = &'a mut Triangle>;
    type IntersectionIterator: Iterator<Item = &'a Triangle>;

    fn new(triangles: Vec<Triangle>) -> Self;
    fn build(&mut self);
    fn intersect(&'a self, ray: Ray, cull: bool) -> Self::IntersectionIterator;
    fn all(&'a self) -> Self::Iterator;
    fn all_mut(&'a mut self) -> Self::IteratorMut;
}
