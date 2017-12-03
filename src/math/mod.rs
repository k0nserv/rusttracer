pub const EPSILON: f32 = 1e-5;

macro_rules! assert_eq_within_bound {
    ($x:expr, $y: expr, $bound: expr) => (
        assert!(
            $x >= $y - $bound && $x <= $y + $bound,
            "{} is not equal to {} within bound {}",
            $x, $y, $bound
        );
    );
}

macro_rules! assert_eq_vector3 {
    ($x:expr, $y: expr, $bound: expr) => (
        assert_eq_within_bound!($x.x, $y.x, $bound);
        assert_eq_within_bound!($x.y, $y.y, $bound);
        assert_eq_within_bound!($x.z, $y.z, $bound);
    );
}

macro_rules! assert_eq_point3 {
    ($x:expr, $y: expr, $bound: expr) => (
        assert_eq_within_bound!($x.x, $y.x, $bound);
        assert_eq_within_bound!($x.y, $y.y, $bound);
        assert_eq_within_bound!($x.z, $y.z, $bound);
    );
}

pub mod vector3;
pub mod matrix4;
pub mod transform;

pub use self::vector3::{Point3, Vector3};
pub use self::matrix4::Matrix4;
pub use self::transform::Transform;
