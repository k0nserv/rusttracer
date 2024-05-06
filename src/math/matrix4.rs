use math::EPSILON;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    // Rows in the outer slice and columns in the inner
    data: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Matrix4 { data }
    }

    pub fn transpose(&self) -> Self {
        Self::new([
            [self[(0, 0)], self[(1, 0)], self[(2, 0)], self[(3, 0)]],
            [self[(0, 1)], self[(1, 1)], self[(2, 1)], self[(3, 1)]],
            [self[(0, 2)], self[(1, 2)], self[(2, 2)], self[(3, 2)]],
            [self[(0, 3)], self[(1, 3)], self[(2, 3)], self[(3, 3)]],
        ])
    }

    pub fn inverse(&self) -> Result<Matrix4, &str> {
        let mut result = Matrix4::identity();
        let mut self_copy = *self;

        for column in 0..4 {
            if self_copy[(column, column)].abs() < EPSILON {
                let mut larger = column;

                for row in 0..4 {
                    if self_copy[(row, column)].abs() > self_copy[(larger, column)].abs() {
                        larger = row;
                    }
                }

                if larger == column {
                    return Err("Singular matrix, cannot be inverted");
                }

                self_copy.data.swap(column, larger);
                result.data.swap(column, larger);
            }

            for row in 0..4 {
                if row == column {
                    continue;
                }

                let coeff = self_copy[(row, column)] / self_copy[(column, column)];

                if coeff != 0.0 {
                    for j in 0..4 {
                        self_copy[(row, j)] -= coeff * self_copy[(column, j)];
                        result[(row, j)] -= coeff * result[(column, j)];
                    }

                    self_copy[(row, column)] = 0.0;
                }
            }
        }

        for row in 0..4 {
            for column in 0..4 {
                result[(row, column)] /= self_copy[(row, row)];
            }
        }

        Ok(result)
    }

    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ])
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale_uniform(scale: f32) -> Self {
        Self::scale(scale, scale, scale)
    }

    pub fn rot_x(theta: f32) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, theta.cos(), theta.sin(), 0.0],
            [0.0, -theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rot_y(theta: f32) -> Self {
        Self::new([
            [theta.cos(), 0.0, -theta.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [theta.sin(), 0.0, theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rot_z(theta: f32) -> Self {
        Self::new([
            [theta.cos(), theta.sin(), 0.0, 0.0],
            [-theta.sin(), theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &[f32; 4] {
        &self.data[index]
    }
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        &mut self.data[index.0][index.1]
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = Self::identity();

        for i in 0..4 {
            for j in 0..4 {
                result[(i, j)] = self[(i, 0)] * other[(0, j)]
                    + self[(i, 1)] * other[(1, j)]
                    + self[(i, 2)] * other[(2, j)]
                    + self[(i, 3)] * other[(3, j)];
            }
        }

        result
    }
}

#[cfg(test)]
macro_rules! assert_eq_matrix4 {
    ($x:expr, $y:expr, $bound:expr) => {
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
    };
}

#[cfg(test)]
mod tests {
    use super::Matrix4;
    use std::f32::consts::PI;

    const EPSILON: f32 = 1e-3;

    #[test]
    fn test_identity() {
        let i = Matrix4::identity();

        let expected = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq_matrix4!(i, expected, EPSILON);
    }

    #[test]
    fn test_new() {
        let m = Matrix4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let expected = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn transpose_identity() {
        let m = Matrix4::identity();

        let expected = Matrix4::identity();

        assert_eq_matrix4!(m.transpose(), expected, EPSILON);
    }

    #[test]
    fn transpose_complex() {
        let m = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let expected = Matrix4::new([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);

        assert_eq_matrix4!(m.transpose(), expected, EPSILON);
    }

    #[test]
    fn inverse_identity() {
        let m = Matrix4::identity();

        let inverse = m.inverse().expect("Identity matrix should be invertible");

        assert_eq_matrix4!(inverse, m, EPSILON);
    }

    #[test]
    fn inverse_moderate() {
        let m = Matrix4::new([
            [2.0, 3.0, 1.0, 5.0],
            [1.0, 0.0, 3.0, 1.0],
            [0.0, 2.0, -3.0, 2.0],
            [0.0, 2.0, 3.0, 1.0],
        ]);

        let expected = Matrix4::new([
            [18.0, -35.0, -28.0, 1.0],
            [9.0, -18.0, -14.0, 1.0],
            [-2.0, 4.0, 3.0, 0.0],
            [-12.0, 24.0, 19.0, -1.0],
        ]);

        assert_eq_matrix4!(m * expected, Matrix4::identity(), 1e-4);

        let inverse = m
            .inverse()
            .unwrap_or_else(|e| panic!("{:?} should be invertible: {}", m, e));

        assert_eq_matrix4!(inverse, expected, 1e-4);
    }

    #[test]
    fn inverse_complex() {
        let matrices = [
            Matrix4::new([
                [2.0, 3.0, 1.0, 5.0],
                [1.0, 0.0, 3.0, 1.0],
                [0.0, 2.0, -3.0, 2.0],
                [0.0, 2.0, 3.0, 1.0],
            ]),
            Matrix4::rot_x(PI / 2.0),
            Matrix4::rot_y(PI / 2.0),
            Matrix4::new([
                [1.0, 1.0, 1.0, 0.0],
                [0.0, 3.0, 1.0, 2.0],
                [2.0, 3.0, 1.0, 0.0],
                [1.0, 0.0, 2.0, 1.0],
            ]),
        ];
        let identity = Matrix4::identity();

        for matrix in matrices.iter() {
            let inverse = matrix
                .inverse()
                .unwrap_or_else(|e| panic!("{:?} should be invertible: {}", matrix, e));
            println!("Testing {:?}", matrix);

            assert_eq_matrix4!((inverse * *matrix), identity, EPSILON);
        }
    }

    #[test]
    fn index_row() {
        let m = Matrix4::identity();

        let rows = [m[0], m[1], m[2], m[3]];

        for (i, item) in rows.iter().enumerate() {
            assert_eq_within_bound!(item[i], 1.0, EPSILON);
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

        let expected = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-2.0, 3.0, 5.0, 1.0],
        ]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_scale() {
        let m = Matrix4::scale(5.0, 3.0, -1.0);

        let expected = Matrix4::new([
            [5.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_rot_x() {
        let m = Matrix4::rot_x(PI / 2.0);

        let expected = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_rot_y() {
        let m = Matrix4::rot_y(PI / 2.0);

        let expected = Matrix4::new([
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }

    #[test]
    fn test_rot_z() {
        let m = Matrix4::rot_z(PI / 2.0);

        let expected = Matrix4::new([
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq_matrix4!(m, expected, EPSILON);
    }
}
