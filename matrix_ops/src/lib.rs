// use matrix::Matrix;
pub use lalgebra_scalar::Scalar;
pub use std::ops::{Add, Sub};


#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result: Vec<Vec<T>> = self.0.iter()
            .zip(other.0.iter())
            .map(|(row_a, row_b)| {
                row_a.iter()
                    .zip(row_b.iter())
                    .map(|(&a, &b)| a + b)
                    .collect()
            })
            .collect();

        Some(Matrix(result))
    }

}

// impl Sub for Matrix {

// }