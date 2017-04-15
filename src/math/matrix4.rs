use std::ops::{Mul, Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    // Rows in the outer slice and columns in the inner
    data: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn new(data: [[f64; 4]; 4]) -> Matrix4 {
        Matrix4 { data: data }
    }

    pub fn identity() -> Matrix4 {
        Self::new([[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.],
                   [0.0, 0.0, 1.0, 0.0],
                   [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Matrix4 {
        Self::new([[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [x, y, z, 1.0]])
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Matrix4 {
        Self::new([[x, 0.0, 0.0, 0.0], [0.0, y, 0.0, 0.], [0.0, 0.0, z, 0.0], [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn rot_x(theta: f64) -> Matrix4 {
        Self::new([[1.0, 0.0, 0.0, 0.0],
                   [0.0, theta.cos(), theta.sin(), 0.0],
                   [0.0, -theta.sin(), theta.cos(), 0.0],
                   [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn rot_y(theta: f64) -> Matrix4 {
        Self::new([[theta.cos(), 0.0, -theta.sin(), 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [theta.sin(), 0.0, theta.cos(), 0.0],
                   [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn rot_z(theta: f64) -> Matrix4 {
        Self::new([[theta.cos(), theta.sin(), 0.0, 0.0],
                   [-theta.sin(), theta.cos(), 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [0.0, 0.0, 0.0, 1.0]])
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &[f64; 4] {
        &self.data[index]
    }
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &f64 {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64 {
        &mut self.data[index.0][index.1]
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = Self::identity();

        for i in 0..4 {
            for j in 0..4 {
                result[(i, j)] = self[(i, 0)] * other[(0, j)] + self[(i, 1)] * other[(1, j)] +
                                 self[(i, 2)] * other[(2, j)] +
                                 self[(i, 3)] * other[(3, j)];
            }
        }

        result
    }
}

macro_rules! assert_eq_matrix4 {
    ($x:expr, $y: expr, $bound: expr) => (
        assert_eq_within_bound!($x[(0, 0)], $y[(0, 0)], $bound);
        assert_eq_within_bound!($x[(0, 1)], $y[(0, 1)], $bound);
        assert_eq_within_bound!($x[(0, 2)], $y[(0, 2)], $bound);
        assert_eq_within_bound!($x[(1, 0)], $y[(1, 0)], $bound);
        assert_eq_within_bound!($x[(1, 1)], $y[(1, 1)], $bound);
        assert_eq_within_bound!($x[(1, 2)], $y[(1, 2)], $bound);
        assert_eq_within_bound!($x[(2, 0)], $y[(2, 0)], $bound);
        assert_eq_within_bound!($x[(2, 1)], $y[(2, 1)], $bound);
        assert_eq_within_bound!($x[(2, 2)], $y[(2, 2)], $bound);
        assert_eq_within_bound!($x[(3, 0)], $y[(3, 0)], $bound);
        assert_eq_within_bound!($x[(3, 1)], $y[(3, 1)], $bound);
        assert_eq_within_bound!($x[(3, 2)], $y[(3, 2)], $bound);
        assert_eq_within_bound!($x[(3, 3)], $y[(3, 3)], $bound);
    );
}

#[cfg(test)]
mod tests {
    use math::EPSILON;
    use super::Matrix4;
    use std::f64::consts::PI;

    #[test]
    fn test_identity() {
        let i = Matrix4::identity();

        let expected = Matrix4::new([[1.0, 0.0, 0.0, 0.0],
                                     [0.0, 1.0, 0.0, 0.0],
                                     [0.0, 0.0, 1.0, 0.0],
                                     [0.0, 0.0, 0.0, 1.0]]);

        assert_eq_matrix4!(i, expected, EPSILON);
    }

    #[test]
    fn test_new() {
        let m = Matrix4 {
            data: [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [0.0, 0.0, 0.0, 1.0]],
        };

        let expected = Matrix4::new([[1.0, 0.0, 0.0, 0.0],
                                     [0.0, 1.0, 0.0, 0.0],
                                     [0.0, 0.0, 1.0, 0.0],
                                     [0.0, 0.0, 0.0, 1.0]]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn index_row() {
        let m = Matrix4::identity();

        let rows = [m[0], m[1], m[2], m[3]];

        for i in 0..4 {
            assert_eq_within_bound!(rows[i][i], 1.0, EPSILON);
        }
    }

    #[test]
    fn index() {
        let m = Matrix4::identity();

        for i in 0..4 {
            assert_eq_within_bound!(m[(i, i)], 1.0, EPSILON);
        }
    }

    #[test]
    fn index_mut() {
        let mut m = Matrix4::identity();

        for i in 0..4 {
            m[(3, i)] = 4.5
        }

        for i in 0..4 {
            assert_eq_within_bound!(m[(3, i)], 4.5, EPSILON);
        }
    }

    #[test]
    fn test_translate() {
        let m = Matrix4::translate(-2.0, 3.0, 5.0);

        let expected = Matrix4::new([[1.0, 0.0, 0.0, 0.0],
                                     [0.0, 1.0, 0.0, 0.0],
                                     [0.0, 0.0, 1.0, 0.0],
                                     [-2.0, 3.0, 5.0, 1.0]]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_scale() {
        let m = Matrix4::scale(5.0, 3.0, -1.0);

        let expected = Matrix4::new([[5.0, 0.0, 0.0, 0.0],
                                     [0.0, 3.0, 0.0, 0.0],
                                     [0.0, 0.0, -1.0, 0.0],
                                     [0.0, 0.0, 0.0, 1.0]]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_rot_x() {
        let m = Matrix4::rot_x(PI / 2.0);


        let expected = Matrix4::new([[1.0, 0.0, 0.0, 0.0],
                                     [0.0, 0.0, 1.0, 0.0],
                                     [0.0, -1.0, 0.0, 0.0],
                                     [0.0, 0.0, 0.0, 1.0]]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_rot_y() {
        let m = Matrix4::rot_y(PI / 2.0);


        let expected = Matrix4::new([[0.0, 0.0, -1.0, 0.0],
                                     [0.0, 1.0, 0.0, 0.0],
                                     [1.0, 0.0, 0.0, 0.0],
                                     [0.0, 0.0, 0.0, 1.0]]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_rot_z() {
        let m = Matrix4::rot_z(PI / 2.0);


        let expected = Matrix4::new([[0.0, 1.0, 0.0, 0.0],
                                     [-1.0, 0.0, 0.0, 0.0],
                                     [0.0, 0.0, 1.0, 0.0],
                                     [0.0, 0.0, 0.0, 1.0]]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }
}
