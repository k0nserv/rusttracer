use std::rc::Rc;

use crate::geometry::mesh::Mesh;
use crate::geometry::{BoundingVolume, Intersectable, Shape, Transformable, TriangleStorage};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::math::{Matrix4, Transform};
use crate::ray::Ray;

pub struct Instance<V, S> {
    mesh: Rc<Mesh<V, S>>,
    model_matrix: Matrix4,
    inverse_model_matrix: Matrix4,
    material: Rc<Material>,
}

impl<'a, V: BoundingVolume, S: TriangleStorage<'a>> Instance<V, S> {
    pub fn new(mesh: Rc<Mesh<V, S>>, material: Rc<Material>) -> Self {
        Self {
            mesh,
            model_matrix: Matrix4::identity(),
            inverse_model_matrix: Matrix4::identity(),
            material,
        }
    }
}

impl<V: BoundingVolume, S: for<'a> TriangleStorage<'a>> Transformable for Instance<V, S> {
    fn transform(&mut self, transform: &Transform) {
        self.model_matrix = self.model_matrix * transform.matrix;
    }

    fn apply_transforms(&mut self, transforms: &[Transform]) {
        let mut world_to_view = self.model_matrix;

        for transform in transforms {
            world_to_view = world_to_view * transform.matrix;
        }

        self.model_matrix = world_to_view;
        self.inverse_model_matrix = world_to_view.inverse().unwrap();
    }
}

impl<V: BoundingVolume, S: for<'a> TriangleStorage<'a>> Intersectable for Instance<V, S> {
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection> {
        let inverse_model_matrix = self.inverse_model_matrix;
        let new_ray = Ray::new(
            inverse_model_matrix * ray.origin,
            (inverse_model_matrix * ray.direction).normalize(),
            Some(ray.medium_refraction),
        );

        self.mesh.as_ref().intersect(new_ray, cull).map(|mut i| {
            i.normal = (inverse_model_matrix.transpose() * i.normal).normalize();
            i.point = self.model_matrix * i.point;
            i.t = (ray.origin - i.point).length().abs();
            i.shape = self;

            i
        })
    }
}

impl<V: BoundingVolume, S: for<'a> TriangleStorage<'a>> Shape for Instance<V, S> {
    fn material(&self) -> &Material {
        &self.material
    }
}
