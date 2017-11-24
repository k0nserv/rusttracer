use geometry::{Intersectable, Material, Transformable, Triangle, AABB};
use geometry::triangle::Normal;
use math::{Point3, Transform};
use ray::Ray;
use intersection::Intersection;

#[derive(Debug)]
pub struct Mesh {
    triangles: Vec<Box<Triangle>>,
    aabb: AABB,
}

impl Mesh {
    pub fn new(triangles: Vec<Box<Triangle>>) -> Self {
        let aabb = Self::calculate_bounding_box(&triangles);
        Self {
            triangles: triangles,
            aabb: aabb,
        }
    }

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




        Self::new(triangles)
    }

    fn from_triangles(vertices: Vec<Point3>, material: Material) -> Vec<Box<Triangle>> {
        assert!(vertices.len() % 3 == 0,
                "Number of vertices should be a multiple of 3");
        (0..vertices.len() / 3)
            .map(|i| {
                let a = vertices[i * 3];
                let b = vertices[i * 3 + 1];
                let c = vertices[i * 3 + 2];
                let ab = a - b;
                let ac = a - c;
                let normal = ab.cross(&ac).normalize();

                Box::new(Triangle::new(vertices[i * 3],
                                       vertices[i * 3 + 1],
                                       vertices[i * 3 + 2],
                                       Normal::Face(normal),
                                       material))
            })
            .collect()
    }

    fn calculate_bounding_box(triangles: &Vec<Box<Triangle>>) -> AABB {
        assert!(triangles.len() > 0, "Creating AABB with 0 vertices");
        if triangles.len() == 0 {
            return AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        }
        let mut min = triangles[0].vertices[0];
        let mut max = triangles[0].vertices[0];

        for triangle in triangles.iter() {
            for vertex in triangle.vertices.iter() {
                // Max
                if vertex.x > max.x {
                    max.x = vertex.x;
                }

                if vertex.y > max.y {
                    max.y = vertex.y;
                }

                if vertex.z > max.z {
                    max.z = vertex.z;
                }

                // Min
                if vertex.x < min.x {
                    min.x = vertex.x;
                }

                if vertex.y < min.y {
                    min.y = vertex.y;
                }

                if vertex.z < min.z {
                    min.z = vertex.z;
                }
            }
        }

        AABB::new(min, max)
    }
}

impl Transformable for Mesh {
    fn transform(&mut self, transform: &Transform) {
        for boxed_triangle in self.triangles.iter_mut() {
            boxed_triangle.as_mut().transform(transform);
        }

        self.aabb = Self::calculate_bounding_box(&self.triangles);
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection> {
        if !self.aabb.intersect(ray) {
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
