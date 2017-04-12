pub struct Matrix3 {
    // Rows in the outer slice and columns in the inner
    data: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn new(data: [[f64; 3]; 3]) -> Matrix3 {
        Matrix3 { data: data }
    }

    pub fn identity() -> Matrix3 {
        Self::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }

    #[inline(always)]
    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }
}

macro_rules! assert_eq_matrix3 {
    ($x:expr, $y: expr, $bound: expr) => (
        assert_eq_within_bound!($x.at(0, 0), $y.at(0, 0), $bound);
        assert_eq_within_bound!($x.at(0, 1), $y.at(0, 1), $bound);
        assert_eq_within_bound!($x.at(0, 2), $y.at(0, 2), $bound);
        assert_eq_within_bound!($x.at(1, 0), $y.at(1, 0), $bound);
        assert_eq_within_bound!($x.at(1, 1), $y.at(1, 1), $bound);
        assert_eq_within_bound!($x.at(1, 2), $y.at(1, 2), $bound);
        assert_eq_within_bound!($x.at(2, 0), $y.at(2, 0), $bound);
        assert_eq_within_bound!($x.at(2, 1), $y.at(2, 1), $bound);
        assert_eq_within_bound!($x.at(2, 2), $y.at(2, 2), $bound);
    );
}

#[cfg(test)]
mod tests {
    use math::EPSILON;
    use super::Matrix3;

    #[test]
    fn test_identity() {
        let i = Matrix3::identity();

        let expected = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

        assert_eq_matrix3!(i, expected, EPSILON);
    }

    #[test]
    fn test_new() {
        let m = Matrix3 { data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

        let expected = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

        assert_eq_matrix3!(m, expected, EPSILON);
    }

    #[test]
    fn test_at() {
        let m = Matrix3::identity();

        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    assert_eq_within_bound!(m.at(i, j), 1.0, EPSILON);
                } else {
                    assert_eq_within_bound!(m.at(i, j), 0.0, EPSILON);
                }
            }
        }
    }

}
