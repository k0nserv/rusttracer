pub const EPSILON: f64 = 1e-5;

use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn at_origin() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }


    pub fn dot(&self, other: &Self) -> f64 {
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
        *self - *normal * (2.0 * self.dot(&normal))
    }

    pub fn length(&self) -> f64 {
        self.dot(&self).sqrt()
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
}

// Operators

macro_rules! define_scalar_add {
    ($T: ty) => (
        impl Add<$T> for Vector3 {
            type Output = Vector3;

            fn add(self, other: $T) -> Vector3 {
                Vector3 {
                    x: self.x + (other as f64),
                    y: self.y + (other as f64),
                    z: self.z + (other as f64),
                }
            }
        }
    )
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

define_scalar_add!(f64);
define_scalar_add!(f32);
define_scalar_add!(i8);
define_scalar_add!(i16);
define_scalar_add!(i32);
define_scalar_add!(i64);
define_scalar_add!(u8);
define_scalar_add!(u16);
define_scalar_add!(u32);
define_scalar_add!(u64);

macro_rules! define_scalar_sub {
    ($T: ty) => (
        impl Sub<$T> for Vector3 {
            type Output = Vector3;

            fn sub(self, other: $T) -> Vector3 {
                Vector3 {
                    x: self.x - (other as f64),
                    y: self.y - (other as f64),
                    z: self.z - (other as f64),
                }
            }
        }
    )
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

define_scalar_sub!(f64);
define_scalar_sub!(f32);
define_scalar_sub!(i8);
define_scalar_sub!(i16);
define_scalar_sub!(i32);
define_scalar_sub!(i64);
define_scalar_sub!(u8);
define_scalar_sub!(u16);
define_scalar_sub!(u32);
define_scalar_sub!(u64);

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
    ($T: ty) => (
        impl Mul<$T> for Vector3 {
            type Output = Vector3;

            fn mul(self, other: $T) -> Vector3 {
                Vector3 {
                    x: self.x * (other as f64),
                    y: self.y * (other as f64),
                    z: self.z * (other as f64),
                }
            }
        }
    )
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

define_scalar_mul!(f64);
define_scalar_mul!(f32);
define_scalar_mul!(i8);
define_scalar_mul!(i16);
define_scalar_mul!(i32);
define_scalar_mul!(i64);
define_scalar_mul!(u8);
define_scalar_mul!(u16);
define_scalar_mul!(u32);
define_scalar_mul!(u64);


macro_rules! assert_eq_within_bound {
    ($x:expr, $y: expr, $bound: expr) => (
        assert!($x >= $y - $bound && $x <= $y + $bound, "{} is not equal to {} within bound {}", $x, $y, $bound);
    );
}

macro_rules! assert_eq_vector3 {
    ($x:expr, $y: expr, $bound: expr) => (
        assert_eq_within_bound!($x.x, $y.x, $bound);
        assert_eq_within_bound!($x.y, $y.y, $bound);
        assert_eq_within_bound!($x.z, $y.z, $bound);
    );
}

#[cfg(test)]
mod tests {
    use super::{Vector3, EPSILON};


    #[test]
    fn test_constructor() {
        let vec = Vector3 {
            x: 2.0,
            y: 1.0,
            z: 0.0,
        };

        assert_eq_within_bound!(vec.x, 2.0, EPSILON);
        assert_eq_within_bound!(vec.y, 1.0, EPSILON);
        assert_eq_within_bound!(vec.z, 0.0, EPSILON);
    }

    #[test]
    fn test_at_origin() {
        let vec = Vector3::at_origin();

        assert_eq_vector3!(vec,
                           Vector3 {
                               x: 0.0,
                               y: 0.0,
                               z: 0.0,
                           },
                           EPSILON);
    }

    #[test]
    fn test_dot() {
        let vec1 = Vector3 {
            x: 3.52,
            y: 8.23,
            z: 29.0,
        };

        let vec2 = Vector3 {
            x: 0.0,
            y: 1.3,
            z: -3.23,
        };

        assert_eq_within_bound!(vec1.dot(&vec2), -82.971, EPSILON);
        assert_eq_within_bound!(vec2.dot(&vec1), -82.971, EPSILON);
    }

    #[test]
    fn test_cross() {
        let vec1 = Vector3 {
            x: 2.4,
            y: 9.3,
            z: -1.3,
        };

        let vec2 = Vector3 {
            x: -2.3,
            y: 2.5,
            z: -3.5,
        };

        let result1 = vec1.cross(&vec2);
        let result2 = vec2.cross(&vec1);

        assert_eq_vector3!(result1,
                           Vector3 {
                               x: -29.3,
                               y: 11.39,
                               z: 27.39,
                           },
                           EPSILON);
        assert_eq_vector3!(result2,
                           Vector3 {
                               x: 29.3,
                               y: -11.39,
                               z: -27.39,
                           },
                           EPSILON);
    }

    #[test]
    fn test_length() {
        let vectors = vec![(Vector3 {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            0.0),
                           (Vector3 {
                                x: 2.3,
                                y: -2.1,
                                z: 2.1,
                            },
                            3.756327994),
                           (Vector3 {
                                x: 1.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            1.0),
                           (Vector3 {
                                x: 0.802,
                                y: 0.267,
                                z: 0.534,
                            },
                            1.0)];

        for (vec, length) in vectors {
            assert_eq_within_bound!(vec.length(), length, EPSILON);
        }
    }

    #[test]
    fn test_normalize_zero_length() {
        let vec = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq_vector3!(vec.normalize(), vec, EPSILON);
    }

    #[test]
    fn test_normalize() {
        let vec = Vector3 {
            x: 4.0,
            y: 63.0,
            z: 0.5,
        };

        let result = vec.normalize();
        let expected = Vector3 {
            x: 0.063362486,
            y: 0.99795915,
            z: 0.007920311,
        };

        assert_eq_vector3!(result, expected, EPSILON);
        assert_eq_within_bound!(result.length(), 1.0, EPSILON);
    }

    #[test]
    fn test_addition() {
        let vec1 = Vector3 {
            x: 1.0,
            y: 5.0,
            z: 3.0,
        };

        let vec2 = Vector3 {
            x: 3.2,
            y: 3.1,
            z: 2.1,
        };

        let expected1 = Vector3 {
            x: 4.2,
            y: 8.1,
            z: 5.1,
        };

        let expected2 = Vector3 {
            x: 11.0,
            y: 15.0,
            z: 13.0,
        };

        assert_eq_vector3!(vec1 + vec2, expected1, EPSILON);

        let result: Vector3 = vec1 + 10;
        assert_eq_vector3!(result, expected2, EPSILON);
    }

    #[test]
    fn test_subtraction() {
        let vec1 = Vector3 {
            x: 1.0,
            y: 5.0,
            z: 3.0,
        };

        let vec2 = Vector3 {
            x: 3.2,
            y: 3.1,
            z: 2.1,
        };

        let expected1 = Vector3 {
            x: -2.2,
            y: 1.9,
            z: 0.9,
        };

        let expected2 = Vector3 {
            x: -19.0,
            y: -15.0,
            z: -17.0,
        };


        assert_eq_vector3!(vec1 - vec2, expected1, EPSILON);

        let result: Vector3 = vec1 - 20.0;
        assert_eq_vector3!(result, expected2, EPSILON);
    }

    #[test]
    fn test_multiplication() {
        let vec1 = Vector3 {
            x: 1.0,
            y: 5.0,
            z: 3.0,
        };

        let vec2 = Vector3 {
            x: 3.2,
            y: 3.1,
            z: 2.1,
        };

        let expected1 = Vector3 {
            x: 3.2,
            y: 15.5,
            z: 6.3,
        };
        let expected2 = Vector3 {
            x: 20.0,
            y: 100.0,
            z: 60.0,
        };

        assert_eq_vector3!(vec1 * vec2, expected1, EPSILON);

        let result: Vector3 = vec1 * 20.0;
        assert_eq_vector3!(result, expected2, EPSILON);
    }
}
