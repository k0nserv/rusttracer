use geometry::{Triangle, Material, Transformable, Intersectable};
use math::{Point3, Transform};
use ray::Ray;
use intersection::Intersection;

#[derive(Debug)]
pub struct Mesh {
    triangles: Vec<Box<Triangle>>,
}

impl Mesh {
    pub fn cube(material: Material) -> Self {
        let vertices = [Point3::new(-1.0, -1.0, 1.0),
                        Point3::new(1.0, -1.0, 1.0),
                        Point3::new(1.0, 1.0, 1.0),
                        Point3::new(-1.0, 1.0, 1.0),
                        Point3::new(-1.0, -1.0, -1.0),
                        Point3::new(1.0, -1.0, -1.0),
                        Point3::new(1.0, 1.0, -1.0),
                        Point3::new(-1.0, 1.0, -1.0)];


        let triangles = Self::from_triangles(vec![vertices[0],
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
                                                  vertices[3]],
                                             material);




        Self { triangles: triangles }
    }

    fn from_triangles(vertices: Vec<Point3>, material: Material) -> Vec<Box<Triangle>> {
        assert!(vertices.len() % 3 == 0,
                "Number of vertices should be a multiple of 3");
        (0..vertices.len() / 3)
            .map(|i| {
                     Box::new(Triangle::new(vertices[i * 3],
                                            vertices[i * 3 + 1],
                                            vertices[i * 3 + 2],
                                            material))
                 })
            .collect()
    }
}

impl Transformable for Mesh {
    fn transform(&mut self, transform: &Transform) {
        for boxed_triangle in self.triangles.iter_mut() {
            boxed_triangle.as_mut().transform(transform);
        }
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let mut nearest_intersection: Option<Intersection> = None;

        for boxed_triangle in self.triangles.iter() {
            let potential_intersection = boxed_triangle.intersect(ray);

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
