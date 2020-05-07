use std::mem;

use super::{BoundingVolume, Ray, Triangle};
use math::Vector3;

const NUM_PLANE_SET_NORMALS: usize = 7;
const PLAN_SET_NORMALS: &'static [Vector3; NUM_PLANE_SET_NORMALS] = &[
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
    Vector3::new(0.0, 0.0, 1.0),
    // 0.577350269 = 3.0_f32.sqrt() / 3.0 but alas `sqrt` isn't a const fn
    // so it cannot be used in this context.
    Vector3::new(0.577350269, 0.577350269, 0.577350269),
    Vector3::new(-0.577350269, 0.577350269, 0.577350269),
    Vector3::new(-0.577350269, -0.577350269, 0.577350269),
    Vector3::new(0.577350269, -0.577350269, 0.577350269),
];

pub struct ExtentVolume {
    distances: [[f32; 2]; NUM_PLANE_SET_NORMALS],
}

impl BoundingVolume for ExtentVolume {
    fn new(triangles: &[Triangle]) -> Self {
        let mut distances = [[std::f32::INFINITY, std::f32::NEG_INFINITY]; NUM_PLANE_SET_NORMALS];

        assert!(
            !triangles.is_empty(),
            "Creating ExtentVolume with 0 vertices"
        );
        if triangles.is_empty() {
            return Self { distances };
        }

        for triangle in triangles {
            for point in &triangle.vertices {
                let vertex = point.as_vector();

                for (normal_idx, plane_normal) in PLAN_SET_NORMALS.iter().enumerate() {
                    let distance = plane_normal.dot(&vertex);

                    distances[normal_idx][0] = f32::min(distances[normal_idx][0], distance);
                    distances[normal_idx][1] = f32::max(distances[normal_idx][1], distance);
                }
            }
        }

        Self { distances }
    }

    fn intersect(&self, ray: Ray) -> bool {
        let origin_vector = ray.origin.as_vector();

        let (precomputed_numerator, precomputed_denominator) = {
            let mut precomputed_numerator: [f32; NUM_PLANE_SET_NORMALS] =
                [std::f32::NAN; NUM_PLANE_SET_NORMALS];
            let mut precomputed_denominator: [f32; NUM_PLANE_SET_NORMALS] =
                [std::f32::NAN; NUM_PLANE_SET_NORMALS];

            for i in 0..NUM_PLANE_SET_NORMALS {
                precomputed_numerator[i] = PLAN_SET_NORMALS[i].dot(&origin_vector);
                precomputed_denominator[i] = PLAN_SET_NORMALS[i].dot(&ray.direction);
            }

            (precomputed_numerator, precomputed_denominator)
        };

        let mut t_near = std::f32::NEG_INFINITY;
        let mut t_far = std::f32::INFINITY;

        for i in 0..NUM_PLANE_SET_NORMALS {
            let mut tn =
                (self.distances[i][0] - precomputed_numerator[i]) / precomputed_denominator[i];
            let mut tf =
                (self.distances[i][1] - precomputed_numerator[i]) / precomputed_denominator[i];

            if precomputed_denominator[i] < 0.0 {
                mem::swap(&mut tn, &mut tf);
            }

            t_near = f32::max(tn, t_near);
            t_far = f32::min(tf, t_far);

            if t_near > t_far {
                return false;
            }
        }

        true
    }
}
