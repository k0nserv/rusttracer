const EPSILON: f64 = 0.001;

use std::ops::{Add};

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Self) -> Self {
        let x0 = self.y * other.z - self.z * other.y;
        let y0 = self.z * other.x - self.x * other.z;
        let z0 = self.x * other.y - self.y * other.x;

        return Vector3 {
            x: x0,
            y: y0,
            z: z0
        };
    }

    pub fn length(&self) -> f64 {
        return self.dot(&self).sqrt();
    }

    pub fn normalize(&self) -> Self {
        let l = self.length();

        if l == 0.0 {
            return Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        }

        return Vector3 { x: self.x / l, y: self.y / l, z: self.z / l };
    }
}

// Operators


// This declares two lifetime references. One for the right hand side(a) and
// another for self(b). Since Add is implemented for &'a Vector this means
// self is actually a Vector3 reference. I think that's how it work anyway...
impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, other: &'b Vector3) -> Vector3 {
        return Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z };
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        return &self + &other;
    }
}



#[cfg(test)]
mod tests {
    use super::Vector3;
    use super::EPSILON;

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

    #[test]
    fn test_constructor() {
        let vec = Vector3 {
            x: 2.0,
            y: 1.0,
            z: 0.0
        };

        assert_eq_within_bound!(vec.x, 2.0, EPSILON);
        assert_eq_within_bound!(vec.y, 1.0, EPSILON);
        assert_eq_within_bound!(vec.z, 0.0, EPSILON);
    }

    #[test]
    fn test_dot() {
        let vec1 = Vector3 {
            x: 3.52,
            y: 8.23,
            z: 29.0
        };

        let vec2 = Vector3 {
            x: 0.0,
            y: 1.3,
            z: -3.23
        };

        assert_eq_within_bound!(vec1.dot(&vec2), -82.971, EPSILON);
        assert_eq_within_bound!(vec2.dot(&vec1), -82.971, EPSILON);
    }

    #[test]
    fn test_cross() {
        let vec1 = Vector3 {
            x: 2.4,
            y: 9.3,
            z: -1.3
        };

        let vec2 = Vector3 {
            x: -2.3,
            y: 2.5,
            z: -3.5
        };

       let result1 = vec1.cross(&vec2);
       let result2 = vec2.cross(&vec1);

       assert_eq_vector3!(result1, Vector3 { x: -29.3, y: 11.39, z: 27.39 }, EPSILON);
       assert_eq_vector3!(result2, Vector3 { x: 29.3, y: -11.39, z: -27.39 }, EPSILON);
    }

    #[test]
    fn test_length() {
        let vectors = vec![
            (Vector3 { x: 0.0, y: 0.0, z: 0.0 }, 0.0),
            (Vector3 { x: 2.3, y: -2.1, z: 2.1 }, 3.756327994),
            (Vector3 { x: 1.0, y: 0.0, z: 0.0 }, 1.0),
            (Vector3 { x: 0.802, y: 0.267, z: 0.534 }, 1.0)
        ];

        for (vec, length) in vectors {
            assert_eq_within_bound!(vec.length(), length, EPSILON);
        }
    }

    #[test]
    fn test_normalize_zero_length() {
        let vec = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };

        assert_eq_vector3!(vec.normalize(), vec, EPSILON);
    }

    #[test]
    fn test_normalize() {
        let vec = Vector3 {
            x: 4.0,
            y: 63.0,
            z: 0.5
        };

        let result = vec.normalize();
        let expected = Vector3 { x: 0.063362486, y: 0.99795915, z: 0.007920311 };

        assert_eq_vector3!(result, expected, EPSILON);
        assert_eq_within_bound!(result.length(), 1.0, EPSILON);
    }
}
