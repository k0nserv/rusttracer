use std::rc::Rc;

use geometry::mesh::Mesh;
use geometry::{BoundingVolume, Intersectable, Shape, Transformable, TriangleStorage};
use intersection::Intersection;
use material::Material;
use math::{Matrix4, Transform};
use ray::Ray;

pub struct Instance<V, S> {
    mesh: Rc<Mesh<V, S>>,
    world_to_view: Matrix4,
    material: Rc<Material>,
}

impl<'a, V: BoundingVolume, S: TriangleStorage<'a>> Instance<V, S> {
    pub fn new(mesh: Rc<Mesh<V, S>>, material: Rc<Material>) -> Self {
        Self {
            mesh,
            world_to_view: Matrix4::identity(),
            material,
        }
    }
}

impl<V: BoundingVolume, S: for<'a> TriangleStorage<'a>> Transformable for Instance<V, S> {
    fn transform(&mut self, transform: &Transform) {
        self.world_to_view = self.world_to_view * transform.matrix;
    }

    fn apply_transforms(&mut self, transforms: &[Transform]) {
        let mut world_to_view = self.world_to_view;

        for transform in transforms {
            world_to_view = world_to_view * transform.matrix;
        }

        self.world_to_view = world_to_view;
    }
}

impl<V: BoundingVolume, S: for<'a> TriangleStorage<'a>> Intersectable for Instance<V, S> {
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection> {
        let inverse_world_to_view = self.world_to_view.inverse().unwrap();
        let new_ray = Ray::new(
            ray.origin * inverse_world_to_view,
            (ray.direction * inverse_world_to_view).normalize(),
            Some(ray.medium_refraction),
        );

        self.mesh.as_ref().intersect(new_ray, cull).map(|mut i| {
            i.normal = (i.normal * inverse_world_to_view.transpose()).normalize();
            i.point = i.point * self.world_to_view;
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
