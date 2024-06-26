use std::ops::{Add, Mul, Neg, Sub};

use serde::Deserialize;

use super::Matrix4;

macro_rules! define_struct {
    ($T:ident) => {
        #[derive(Debug, Copy, Clone, Deserialize)]
        pub struct $T {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }
    };
}

define_struct!(Vector3);
define_struct!(Point3);

macro_rules! define_impl {
    ($T:ident) => {
        impl $T {
            pub const fn new(x: f32, y: f32, z: f32) -> $T {
                $T { x, y, z }
            }

            pub fn fuzzy_equal(&self, other: &Point3) -> bool {
                self.x >= other.x - 1e-3
                    && self.x <= other.x + 1e-3
                    && self.y >= other.y - 1e-3
                    && self.y <= other.y + 1e-3
                    && self.z >= other.z - 1e-3
                    && self.z <= other.z + 1e-3
            }
        }
    };
}

define_impl!(Vector3);
define_impl!(Point3);

macro_rules! define_from {
    ($T:ident) => {
        impl From<[f32; 3]> for $T {
            fn from(values: [f32; 3]) -> $T {
                $T::new(values[0], values[1], values[2])
            }
        }

        impl From<Vec<f32>> for $T {
            fn from(values: Vec<f32>) -> $T {
                assert!(values.len() == 3, "Invalid value for from");
                $T::new(values[0], values[1], values[2])
            }
        }
    };
}

define_from!(Vector3);
define_from!(Point3);

// Vector 3 specific
impl Vector3 {
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        let x0 = self.y * other.z - self.z * other.y;
        let y0 = self.z * other.x - self.x * other.z;
        let z0 = self.x * other.y - self.y * other.x;

        Vector3 {
            x: x0,
            y: y0,
            z: z0,
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * (2.0 * self.dot(normal))
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let l = self.length();

        if l == 0.0 {
            return Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        Vector3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn as_point(&self) -> Point3 {
        Point3::new(self.x, self.y, self.z)
    }
}

// Point3 specific
impl Point3 {
    pub fn at_origin() -> Point3 {
        Point3::new(0.0, 0.0, 0.0)
    }

    pub fn as_vector(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

// Operators
macro_rules! define_scalar_add {
    ($T:ty) => {
        impl Add<$T> for Vector3 {
            type Output = Vector3;

            fn add(self, other: $T) -> Vector3 {
                Vector3 {
                    x: self.x + f32::from(other),
                    y: self.y + f32::from(other),
                    z: self.z + f32::from(other),
                }
            }
        }
    };
}

macro_rules! define_add {
    ($T:ident, $V:ident, $U:ident) => {
        impl Add<$V> for $T {
            type Output = $U;

            fn add(self, other: $V) -> $U {
                $U {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }
    };
}

define_add!(Vector3, Vector3, Vector3);
define_add!(Point3, Vector3, Vector3);

define_scalar_add!(f32);
define_scalar_add!(i8);
define_scalar_add!(i16);
define_scalar_add!(u8);
define_scalar_add!(u16);

macro_rules! define_scalar_sub {
    ($T:ty) => {
        impl Sub<$T> for Vector3 {
            type Output = Vector3;

            fn sub(self, other: $T) -> Vector3 {
                Vector3 {
                    x: self.x - f32::from(other),
                    y: self.y - f32::from(other),
                    z: self.z - f32::from(other),
                }
            }
        }
    };
}

macro_rules! define_sub {
    ($T:ident, $V:ident, $U:ident) => {
        impl Sub<$V> for $T {
            type Output = $U;

            fn sub(self, other: $V) -> $U {
                $U {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }
    };
}

define_sub!(Point3, Point3, Vector3);
define_sub!(Vector3, Vector3, Vector3);

define_scalar_sub!(f32);
define_scalar_sub!(i8);
define_scalar_sub!(i16);
define_scalar_sub!(u8);
define_scalar_sub!(u16);

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

macro_rules! define_scalar_mul {
    ($T:ty) => {
        impl Mul<$T> for Vector3 {
            type Output = Vector3;

            fn mul(self, other: $T) -> Vector3 {
                Vector3 {
                    x: self.x * f32::from(other),
                    y: self.y * f32::from(other),
                    z: self.z * f32::from(other),
                }
            }
        }
    };
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

define_scalar_mul!(f32);
define_scalar_mul!(i8);
define_scalar_mul!(i16);
define_scalar_mul!(u8);
define_scalar_mul!(u16);

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self[(0, 0)] * other.x + self[(1, 0)] * other.y + self[(2, 0)] * other.z,
            self[(0, 1)] * other.x + self[(1, 1)] * other.y + self[(2, 1)] * other.z,
            self[(0, 2)] * other.x + self[(1, 2)] * other.y + self[(2, 2)] * other.z,
        )
    }
}

impl Mul<Point3> for Matrix4 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        let mut x =
            self[(0, 0)] * other.x + self[(1, 0)] * other.y + self[(2, 0)] * other.z + self[(3, 0)];
        let mut y =
            self[(0, 1)] * other.x + self[(1, 1)] * other.y + self[(2, 1)] * other.z + self[(3, 1)];
        let mut z =
            self[(0, 2)] * other.x + self[(1, 2)] * other.y + self[(2, 2)] * other.z + self[(3, 2)];
        let w =
            self[(0, 3)] * other.x + self[(1, 3)] * other.y + self[(2, 3)] * other.z + self[(3, 3)];

        if !(w > (0.0 - 1e-3) && w < (0.0 + 1e-3) || w > (1.0 - 1e-3) && w < (1.0 + 1e-3)) {
            // assert!(false, "Bad value for w {}", w);
            x /= w;
            y /= w;
            z /= w;
        }

        Point3::new(x, y, z)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    const EPSILON: f32 = 1e-3;

    #[test]
    fn test_constructor() {
        let vec = Vector3::new(2.0, 1.0, 0.0);

        assert_eq_within_bound!(vec.x, 2.0, EPSILON);
        assert_eq_within_bound!(vec.y, 1.0, EPSILON);
        assert_eq_within_bound!(vec.z, 0.0, EPSILON);
    }

    #[test]
    fn test_at_origin() {
        let vec = Point3::at_origin();

        assert_eq_point3!(
            vec,
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            EPSILON
        );
    }

    #[test]
    fn test_dot() {
        let vec1 = Vector3::new(3.52, 8.23, 29.0);

        let vec2 = Vector3::new(0.0, 1.3, -3.23);

        assert_eq_within_bound!(vec1.dot(&vec2), -82.971, EPSILON);
        assert_eq_within_bound!(vec2.dot(&vec1), -82.971, EPSILON);
    }

    #[test]
    fn test_cross() {
        let vec1 = Vector3::new(2.4, 9.3, -1.3);

        let vec2 = Vector3::new(-2.3, 2.5, -3.5);

        let result1 = vec1.cross(&vec2);
        let result2 = vec2.cross(&vec1);

        assert_eq_vector3!(result1, Vector3::new(-29.3, 11.39, 27.39,), EPSILON);
        assert_eq_vector3!(result2, Vector3::new(29.3, -11.39, -27.39,), EPSILON);
    }

    #[test]
    fn test_length() {
        let vectors = vec![
            (Vector3::new(0.0, 0.0, 0.0), 0.0),
            (Vector3::new(2.3, -2.1, 2.1), 3.756_328),
            (Vector3::new(1.0, 0.0, 0.0), 1.0),
            (Vector3::new(0.80181, 0.26921, 0.53351), 1.0),
        ];

        for (vec, length) in vectors {
            assert_eq_within_bound!(vec.length(), length, EPSILON);
        }
    }

    #[test]
    fn test_normalize_zero_length() {
        let vec = Vector3::new(0.0, 0.0, 0.0);

        assert_eq_vector3!(vec.normalize(), vec, EPSILON);
    }

    #[test]
    fn test_normalize() {
        let vec = Vector3::new(4.0, 63.0, 0.5);

        let result = vec.normalize();
        let expected = Vector3::new(0.063_362_49, 0.99795915, 0.007920311);

        assert_eq_vector3!(result, expected, EPSILON);
        assert_eq_within_bound!(result.length(), 1.0, EPSILON);
    }

    #[test]
    fn test_addition() {
        let vec1 = Vector3::new(1.0, 5.0, 3.0);

        let vec2 = Vector3::new(3.2, 3.1, 2.1);

        let expected1 = Vector3::new(4.2, 8.1, 5.1);

        let expected2 = Vector3::new(11.0, 15.0, 13.0);

        assert_eq_vector3!(vec1 + vec2, expected1, EPSILON);

        let result: Vector3 = vec1 + 10u16;
        assert_eq_vector3!(result, expected2, EPSILON);
    }

    #[test]
    fn test_subtraction() {
        let vec1 = Vector3::new(1.0, 5.0, 3.0);

        let vec2 = Vector3::new(3.2, 3.1, 2.1);

        let expected1 = Vector3::new(-2.2, 1.9, 0.9);

        let expected2 = Vector3::new(-19.0, -15.0, -17.0);

        assert_eq_vector3!(vec1 - vec2, expected1, EPSILON);

        let result: Vector3 = vec1 - 20.0;
        assert_eq_vector3!(result, expected2, EPSILON);
    }

    #[test]
    fn test_multiplication() {
        let vec1 = Vector3::new(1.0, 5.0, 3.0);

        let vec2 = Vector3::new(3.2, 3.1, 2.1);

        let expected1 = Vector3::new(3.2, 15.5, 6.3);
        let expected2 = Vector3::new(20.0, 100.0, 60.0);

        assert_eq_vector3!(vec1 * vec2, expected1, EPSILON);

        let result: Vector3 = vec1 * 20.0;
        assert_eq_vector3!(result, expected2, EPSILON);
    }

    #[test]
    fn test_vector3_mul_simple() {
        let m = Matrix4::identity();

        let result = m * Vector3::new(2.4, 3.1, 9.0);

        assert_eq_vector3!(result, Vector3::new(2.4, 3.1, 9.0), EPSILON);
    }

    #[test]
    fn test_vector3_mul_complex() {
        let m = Matrix4::new([
            [15.0, 1.3, -2.8, 0.0],
            [-1.4, 7.8, 3.5, 0.0],
            [5.0, -3.6, 1.0, 0.0],
            [12.3, 9.1, -1.2, 1.0],
        ]);

        let result = m * Vector3::new(2.4, 3.2, -1.0);

        assert_eq_vector3!(result, Vector3::new(26.52, 31.68, 3.48), EPSILON);
    }

    #[test]
    fn test_translation() {
        let v = Point3::new(1.5, 9.9, -5.6);
        let m = Matrix4::translate(-2.0, 3.0, 5.0);

        let expected = Point3::new(&v.x - 2.0, &v.y + 3.0, &v.z + 5.0);

        assert_eq_point3!(m * v, expected, EPSILON);
    }

    #[test]
    fn test_vector_translation() {
        let v = Vector3::new(0.0, 0.0, -1.0);
        let m = Matrix4::translate(10.0, 0.0, 0.0);

        let expected = Point3::new(0.0, 0.0, -1.0);

        assert_eq_point3!(m * v, expected, EPSILON);
    }

    #[test]
    fn test_scale() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        let m = Matrix4::scale(-2.0, 3.0, 5.0);

        let expected = Vector3::new(v.x * -2.0, v.y * 3.0, v.z * 5.0);

        assert_eq_vector3!(m * v, expected, EPSILON);
    }

    #[test]
    fn test_rot_x() {
        let v = Vector3::new(1.0, 0.0, 1.0);
        let m = Matrix4::rot_x(PI / 2.0);

        assert_eq_vector3!(Vector3::new(1.0, -1.0, 0.0), m * v, EPSILON);
    }

    #[test]
    fn test_rot_y() {
        let v = Vector3::new(0.0, 1.0, 1.0);
        let m = Matrix4::rot_y(PI / 2.0);

        assert_eq_vector3!(Vector3::new(1.0, 1.0, 0.0), m * v, EPSILON);
    }

    #[test]
    fn test_rot_z() {
        let v = Vector3::new(1.0, 0.0, 1.0);
        let m = Matrix4::rot_z(PI / 2.0);

        assert_eq_vector3!(Vector3::new(0.0, 1.0, 1.0), m * v, EPSILON);
    }

    #[test]
    fn test_from() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let p = Point3::new(1.0, 2.0, 3.0);

        assert_eq_vector3!(Vector3::from([1.0, 2.0, 3.0]), v, EPSILON);
        assert_eq_vector3!(Vector3::from(vec![1.0, 2.0, 3.0]), v, EPSILON);

        assert_eq_point3!(Point3::from([1.0, 2.0, 3.0]), p, EPSILON);
        assert_eq_point3!(Point3::from(vec![1.0, 2.0, 3.0]), p, EPSILON);
    }
}
