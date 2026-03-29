use na::Vector4;
use nalgebra as na;
use std::{cmp, fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub data: Vector4<f64>,
}

const EPSILON: f64 = 0.00001;

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:.4}, {:.4}, {:.4}, {:.4})",
            self.data.x, self.data.y, self.data.z, self.data.w
        )
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple {
            data: Vector4::new(x, y, z, w),
        }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple {
            data: Vector4::new(x, y, z, 1.0),
        }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple {
            data: Vector4::new(x, y, z, 0.0),
        }
    }

    pub fn is_point(&self) -> bool {
        self.data.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.data.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.data.x.powi(2) + self.data.y.powi(2) + self.data.z.powi(2) + self.data.w.powi(2))
            .sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        self.to_owned() / mag
    }

    pub fn dot(v1: &Self, v2: &Self) -> f64 {
        v1.data.x * v2.data.x
            + v1.data.y * v2.data.y
            + v1.data.z * v2.data.z
            + v1.data.w * v2.data.w
    }

    pub fn cross(v1: &Self, v2: &Self) -> Self {
        Tuple::new_vector(
            v1.data.y * v2.data.z - v1.data.z * v2.data.y,
            v1.data.z * v2.data.x - v1.data.x * v2.data.z,
            v1.data.x * v2.data.y - v1.data.y * v2.data.x,
        )
    }
}

impl cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (self.data.x - other.data.x).abs() < EPSILON
            && (self.data.y - other.data.y).abs() < EPSILON
            && (self.data.z - other.data.z).abs() < EPSILON
            && (self.data.w - other.data.w).abs() < EPSILON
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.data.x + rhs.data.x,
            self.data.y + rhs.data.y,
            self.data.z + rhs.data.z,
            self.data.w + rhs.data.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.data.x - rhs.data.x,
            self.data.y - rhs.data.y,
            self.data.z - rhs.data.z,
            self.data.w - rhs.data.w,
        )
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(
            self.data.x * rhs,
            self.data.y * rhs,
            self.data.z * rhs,
            self.data.w * rhs,
        )
    }
}

impl ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Self::Output::new(
            self * rhs.data.x,
            self * rhs.data.y,
            self * rhs.data.z,
            self * rhs.data.w,
        )
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(
            self.data.x / rhs,
            self.data.y / rhs,
            self.data.z / rhs,
            self.data.w / rhs,
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.data.x, -self.data.y, -self.data.z, -self.data.w)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn is_a_point() {
        let point = Tuple::new_point(4.3, -4.2, 3.1);
        assert_eq!(point.data.x, 4.3);
        assert_eq!(point.data.y, -4.2);
        assert_eq!(point.data.z, 3.1);
        assert_eq!(point.data.w, 1.0);

        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn is_a_vector() {
        let vector = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(vector.data.x, 4.3);
        assert_eq!(vector.data.y, -4.2);
        assert_eq!(vector.data.z, 3.1);
        assert_eq!(vector.data.w, 0.0);

        assert!(!vector.is_point());
        assert!(vector.is_vector());
    }

    #[test]
    fn create_a_point() {
        let point = Tuple::new_point(4.3, -4.2, 3.1);
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(point, tuple);
    }

    #[test]
    fn create_a_vector() {
        let vector = Tuple::new_vector(4.3, -4.2, 3.1);
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(vector, tuple);
    }

    #[test]
    fn tuple_addition() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let res = a1 + a2;

        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
        assert_eq!(res, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn tuple_subtraction_two_points() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);

        let res = p1 - p2;

        assert_eq!(p1 - p2, Tuple::new_vector(-2.0, -4.0, -6.0));
        assert_eq!(res, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn tuple_subtraction_vector_from_point() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);

        let res = p - v;
        let expected_res = Tuple::new_point(-2.0, -4.0, -6.0);

        assert_eq!(p - v, expected_res);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn tuple_subtracting_vector_from_zero_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, -v);
    }

    #[test]
    fn tuple_negating_a_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-t, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiply_a_tuple_by_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(3.5 * t, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiply_a_tuple_by_fraction() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(t * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(t / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_of_vector() {
        let t = Tuple::new_vector(1.0, 0.0, 0.0);
        assert_eq!(t.magnitude(), 1.0);

        let t = Tuple::new_vector(0.0, 1.0, 0.0);
        assert_eq!(t.magnitude(), 1.0);

        let t = Tuple::new_vector(0.0, 0.0, 1.0);
        assert_eq!(t.magnitude(), 1.0);

        let t = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(t.magnitude(), 14.0_f64.sqrt());

        let t = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert_eq!(t.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizing_vector() {
        let v = Tuple::new_vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::new_vector(1.0, 0.0, 0.0));

        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let mag = v.magnitude();
        assert_eq!(v.normalize(), v / mag);

        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product() {
        let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::dot(&v1, &v2), 20.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::cross(&v1, &v2), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(Tuple::cross(&v2, &v1), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}
