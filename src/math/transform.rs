use super::Matrix4;

pub struct Transform {
    pub matrix: Matrix4,
    pub normal_matrix: Matrix4,
}

impl Transform {
    pub fn new(matrix: Matrix4) -> Self {
        Self {
            matrix,
            normal_matrix: matrix.transpose(),
        }
    }
}
