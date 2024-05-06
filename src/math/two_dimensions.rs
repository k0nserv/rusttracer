use std::ops::{Add, Mul, Neg, Sub};

macro_rules! define_struct {
    ($T:ident) => {
        #[derive(Debug, Copy, Clone, Deserialize)]
        pub struct $T {
            pub x: f32,
            pub y: f32,
        }
    };
}

define_struct!(Vector2);
define_struct!(Point2);

macro_rules! define_impl {
    ($T:ident) => {
        impl $T {
            pub fn new(x: f32, y: f32) -> $T {
                $T { x, y }
            }
        }
    };
}

define_impl!(Vector2);
define_impl!(Point2);

macro_rules! define_from {
    ($T:ident) => {
        impl From<[f32; 2]> for $T {
            fn from(values: [f32; 2]) -> $T {
                $T::new(values[0], values[1])
            }
        }

        impl From<Vec<f32>> for $T {
            fn from(values: Vec<f32>) -> $T {
                assert!(values.len() == 2, "Invalid value for from");
                $T::new(values[0], values[1])
            }
        }
    };
}

define_from!(Vector2);
define_from!(Point2);

// Vector 3 specific
impl Vector2 {
    pub fn dot(self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(self) -> f32 {
        self.dot(&self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();

        if l == 0.0 {
            return Vector2 { x: 0.0, y: 0.0 };
        }

        Vector2 {
            x: self.x / l,
            y: self.y / l,
        }
    }

    pub fn as_point(self) -> Point2 {
        Point2::new(self.x, self.y)
    }
}

// Point2 specific
impl Point2 {
    pub fn at_origin() -> Point2 {
        Point2::new(0.0, 0.0)
    }

    pub fn as_vector(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

// Operators

macro_rules! define_scalar_add {
    ($T:ty) => {
        impl Add<$T> for Vector2 {
            type Output = Vector2;

            fn add(self, other: $T) -> Vector2 {
                Vector2 {
                    x: self.x + f32::from(other),
                    y: self.y + f32::from(other),
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
                }
            }
        }
    };
}

define_add!(Vector2, Vector2, Vector2);
define_add!(Point2, Vector2, Vector2);

define_scalar_add!(f32);
define_scalar_add!(i8);
define_scalar_add!(i16);
define_scalar_add!(u8);
define_scalar_add!(u16);

macro_rules! define_scalar_sub {
    ($T:ty) => {
        impl Sub<$T> for Vector2 {
            type Output = Vector2;

            fn sub(self, other: $T) -> Vector2 {
                Vector2 {
                    x: self.x - f32::from(other),
                    y: self.y - f32::from(other),
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
                }
            }
        }
    };
}

define_sub!(Point2, Point2, Vector2);
define_sub!(Vector2, Vector2, Vector2);

define_scalar_sub!(f32);
define_scalar_sub!(i8);
define_scalar_sub!(i16);
define_scalar_sub!(u8);
define_scalar_sub!(u16);

macro_rules! define_scalar_mul {
    ($T:ty) => {
        impl Mul<$T> for Vector2 {
            type Output = Vector2;

            fn mul(self, other: $T) -> Vector2 {
                Vector2 {
                    x: self.x * f32::from(other),
                    y: self.y * f32::from(other),
                }
            }
        }
    };
}

impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

define_scalar_mul!(f32);
define_scalar_mul!(i8);
define_scalar_mul!(i16);
define_scalar_mul!(u8);
define_scalar_mul!(u16);

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::EPSILON;
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector2::new(2.0, 1.0);
        let p = Point2::new(2.0, 1.0);

        assert_eq_within_bound!(v.x, 2.0, EPSILON);
        assert_eq_within_bound!(v.y, 1.0, EPSILON);

        assert_eq_within_bound!(p.x, 2.0, EPSILON);
        assert_eq_within_bound!(p.y, 1.0, EPSILON);
    }

    // Vector2 specifics
    #[test]
    fn test_vector2_as_point() {
        let v = Vector2::new(3.0, 1.5);

        assert_eq_point2!(v.as_point(), Point2 { x: 3.0, y: 1.5 }, EPSILON);
    }

    #[test]
    fn test_vector2_length() {
        let v = Vector2::new(29.2, 12.0);

        assert_eq_within_bound!(v.length(), 31.569_605, EPSILON);
    }

    #[test]
    fn test_vector2_dot() {
        let v1 = Vector2::new(29.2, 12.0);
        let v2 = Vector2::new(2.5, 0.0);

        assert_eq_within_bound!(v1.dot(&v2), 73.0, EPSILON);
    }

    #[test]
    fn test_vector2_normalize() {
        let v1 = Vector2::new(29.2, 12.0);

        let normalized = v1.normalize();

        assert_eq_within_bound!(normalized.length(), 1.0, EPSILON);
        assert_eq_vector2!(
            normalized,
            Vector2 {
                x: 0.924_940_3,
                y: 0.380_112_44
            },
            EPSILON
        );
    }

    #[test]
    fn test_vector2_addition() {
        let v1 = Vector2::new(2.0, 3.0);
        let v2 = Vector2::new(1.5, 9.0);

        assert_eq_vector2!(v1 + v2, Vector2 { x: 3.5, y: 12.0 }, EPSILON);
        assert_eq_vector2!(v1 + 10u16, Vector2 { x: 12.0, y: 13.0 }, EPSILON);
    }

    #[test]
    fn test_vector2_substraction() {
        let v1 = Vector2::new(2.0, 3.0);
        let v2 = Vector2::new(1.5, 9.0);

        assert_eq_vector2!(v1 - v2, Vector2 { x: 0.5, y: -6.0 }, EPSILON);
        assert_eq_vector2!(v1 - 10u16, Vector2 { x: -8.0, y: -7.0 }, EPSILON);
    }

    #[test]
    fn test_vector2_multiplication() {
        let v1 = Vector2::new(2.0, 3.0);
        let v2 = Vector2::new(1.5, 9.0);

        assert_eq_vector2!(v1 * v2, Vector2 { x: 3.0, y: 27.0 }, EPSILON);
        assert_eq_vector2!(v1 * 3.0, Vector2 { x: 6.0, y: 9.0 }, EPSILON);
    }

    #[test]
    fn test_vector2_negation() {
        let v = Vector2::new(29.2, 12.0);

        assert_eq_vector2!(-v, Vector2 { x: -29.2, y: -12.0 }, EPSILON);
    }

    // Point2 specifics
    #[test]
    fn test_point2_at_origin() {
        let p = Point2::at_origin();

        assert_eq_point2!(p, Point2 { x: 0.0, y: 0.0 }, EPSILON);
    }

    #[test]
    fn test_point2_as_vector() {
        let p = Point2::new(3.0, 1.5);

        assert_eq_vector2!(p.as_vector(), Vector2 { x: 3.0, y: 1.5 }, EPSILON);
    }

    #[test]
    fn test_point2_addition() {
        let p = Point2::new(3.0, 1.5);
        let v = Vector2::new(2.0, 3.0);

        assert_eq_vector2!(p + v, Vector2 { x: 5.0, y: 4.5 }, EPSILON);
    }

    #[test]
    fn test_from() {
        let p = Point2::new(3.0, 1.5);
        let v = Vector2::new(2.0, 3.0);

        assert_eq_vector2!(Vector2::from([2.0, 3.0]), v, EPSILON);
        assert_eq_vector2!(Vector2::from(vec![2.0, 3.0]), v, EPSILON);

        assert_eq_point2!(Point2::from([3.0, 1.5]), p, EPSILON);
        assert_eq_point2!(Point2::from(vec![3.0, 1.5]), p, EPSILON);
    }
}
