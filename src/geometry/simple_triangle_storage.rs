use super::{Transformable, Triangle, TriangleStorage};
use math::Transform;
use ray::Ray;

#[derive(Debug)]
pub struct SimpleTriangleStorage {
    triangles: Vec<Triangle>,
}

impl<'a> TriangleStorage<'a> for SimpleTriangleStorage {
    type Iterator = std::slice::Iter<'a, Triangle>;
    type IteratorMut = std::slice::IterMut<'a, Triangle>;

    fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
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
