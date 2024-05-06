use std::rc::Rc;

use geometry::triangle::Normal;
use geometry::{BoundingVolume, Intersectable, Material, Transformable, Triangle, TriangleStorage};
use intersection::Intersection;
use math::{Point3, Transform};
use ray::Ray;

#[derive(Debug)]
pub struct Mesh<V, S> {
    storage: S,
    bounding_volume: V,
}

impl<'a, V: BoundingVolume, S: TriangleStorage<'a>> Mesh<V, S> {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let bounding_volume = V::from_triangles(&mut triangles.iter());
        let storage = S::new(triangles);

        Self {
            storage,
            bounding_volume,
        }
    }

    pub fn rebuild_accelleration_structure(&mut self) {
        self.storage.build();
    }

    pub fn cube(material: Rc<Material>) -> Self {
        let vertices = [
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(-1.0, 1.0, 1.0),
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(1.0, 1.0, -1.0),
            Point3::new(-1.0, 1.0, -1.0),
        ];

        let triangles = Self::from_triangles(
            &[
                vertices[0],
                vertices[1],
                vertices[2],
                vertices[2],
                vertices[3],
                vertices[0],
                vertices[1],
                vertices[5],
                vertices[6],
                vertices[6],
                vertices[2],
                vertices[1],
                vertices[7],
                vertices[6],
                vertices[5],
                vertices[5],
                vertices[4],
                vertices[7],
                vertices[4],
                vertices[0],
                vertices[3],
                vertices[3],
                vertices[7],
                vertices[4],
                vertices[4],
                vertices[5],
                vertices[1],
                vertices[1],
                vertices[0],
                vertices[4],
                vertices[3],
                vertices[2],
                vertices[6],
                vertices[6],
                vertices[7],
                vertices[3],
            ],
            material,
        );

        Self::new(triangles)
    }

    fn from_triangles(vertices: &[Point3], material: Rc<Material>) -> Vec<Triangle> {
        assert!(
            vertices.len() % 3 == 0,
            "Number of vertices should be a multiple of 3"
        );
        (0..vertices.len() / 3)
            .map(|i| {
                let a = vertices[i * 3];
                let b = vertices[i * 3 + 1];
                let c = vertices[i * 3 + 2];
                let ab = a - b;
                let ac = a - c;
                let normal = ab.cross(&ac).normalize();

                Triangle::new(
                    vertices[i * 3],
                    vertices[i * 3 + 1],
                    vertices[i * 3 + 2],
                    Normal::Face(normal),
                    None,
                    Rc::clone(&material),
                )
            })
            .collect()
    }
}

impl<V: BoundingVolume, S: for<'a> TriangleStorage<'a>> Transformable for Mesh<V, S> {
    fn transform(&mut self, transform: &Transform) {
        self.storage.transform(transform);

        self.bounding_volume = V::from_triangles(&mut self.storage.all());
    }

    fn apply_transforms(&mut self, transforms: &[Transform]) {
        self.storage.apply_transforms(transforms);
        self.bounding_volume = V::from_triangles(&mut self.storage.all());
    }
}

impl<V: BoundingVolume, S: for<'a> TriangleStorage<'a>> Intersectable for Mesh<V, S> {
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection> {
        if !self.bounding_volume.intersect(ray) {
            return None;
        }

        let mut nearest_intersection: Option<Intersection> = None;

        for triangle in self.storage.intersect(ray, cull) {
            let potential_intersection = triangle.intersect(ray, cull);

            match potential_intersection {
                Some(intersection) => match nearest_intersection {
                    Some(nearest) => {
                        if intersection.t < nearest.t {
                            nearest_intersection = Some(intersection)
                        }
                    }
                    None => nearest_intersection = potential_intersection,
                },
                None => (),
            }
        }

        nearest_intersection
    }
}
