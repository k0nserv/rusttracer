use geometry::triangle::Normal;
use geometry::{BoundingVolume, Intersectable, Material, Transformable, Triangle};
use intersection::Intersection;
use math::{Point3, Transform};
use ray::Ray;

#[derive(Debug)]
pub struct Mesh<T: BoundingVolume> {
    triangles: Vec<Box<Triangle>>,
    bounding_volume: Box<T>,
}

impl<T: BoundingVolume> Mesh<T> {
    pub fn new(triangles: Vec<Box<Triangle>>) -> Self {
        let bounding_volume = Box::new(T::new(&triangles));
        Self {
            triangles,
            bounding_volume,
        }
    }

    pub fn cube(material: Material) -> Self {
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
            vec![
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

    fn from_triangles(vertices: Vec<Point3>, material: Material) -> Vec<Box<Triangle>> {
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

                Box::new(Triangle::new(
                    vertices[i * 3],
                    vertices[i * 3 + 1],
                    vertices[i * 3 + 2],
                    Normal::Face(normal),
                    material,
                ))
            })
            .collect()
    }
}

impl<T: BoundingVolume> Transformable for Mesh<T> {
    fn transform(&mut self, transform: &Transform) {
        for boxed_triangle in self.triangles.iter_mut() {
            boxed_triangle.as_mut().transform(transform);
        }

        self.bounding_volume = Box::new(T::new(&self.triangles));
    }
}

impl<T: BoundingVolume> Intersectable for Mesh<T> {
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection> {
        if !self.bounding_volume.intersect(ray) {
            return None;
        }

        let mut nearest_intersection: Option<Intersection> = None;

        for boxed_triangle in self.triangles.iter() {
            let potential_intersection = boxed_triangle.intersect(ray, cull);

            if let Some(intersection) = potential_intersection {
                if nearest_intersection.is_some() {
                    if intersection.t < nearest_intersection.unwrap().t {
                        nearest_intersection = Some(intersection);
                    }
                } else {
                    nearest_intersection = Some(intersection);
                }
            }
        }

        nearest_intersection
    }
}
