use na::Vector4;
use nalgebra as na;
use std::{cmp, ops};

#[derive(Debug)]
struct Tuple {
    pub data: Vector4<f64>,
}

const EPSILON: f64 = 0.00001;

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
}

impl cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let distance = (self.data.x - other.data.x)
            + (self.data.y - other.data.y)
            + (self.data.z - other.data.z)
            + (self.data.w - other.data.w);
        distance < EPSILON
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
}
