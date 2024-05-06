use super::{Transformable, Triangle, TriangleStorage};
use crate::math::Transform;
use crate::ray::Ray;

#[derive(Debug)]
pub struct SimpleTriangleStorage {
    triangles: Vec<Triangle>,
}

impl Transformable for SimpleTriangleStorage {
    fn transform(&mut self, transform: &Transform) {
        for triangle in self.all_mut() {
            triangle.transform(transform);
        }
    }
}

impl<'a> TriangleStorage<'a> for SimpleTriangleStorage {
    type Iterator = std::slice::Iter<'a, Triangle>;
    type IteratorMut = std::slice::IterMut<'a, Triangle>;
    type IntersectionIterator = std::slice::Iter<'a, Triangle>;

    fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
    }

    fn build(&mut self) {
        // Not much to build when you have no acceleration structure
    }

    fn intersect(&'a self, _ray: Ray, _cull: bool) -> Self::Iterator {
        self.all()
    }

    fn all(&'a self) -> Self::Iterator {
        self.triangles.iter()
    }

    fn all_mut(&'a mut self) -> Self::IteratorMut {
        self.triangles.iter_mut()
    }
}
