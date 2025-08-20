pub use lalgebra_scalar::Scalar;
pub use std::ops::{Mul, Add, Sub, Div};


#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result: Vec<Vec<T>> = self.0.iter()
            .zip(other.0.iter())
            .map(|(row_a, row_b)| {
                row_a.iter()
                    .zip(row_b.iter())
                    .map(|(&a, &b)| a * b)
                    .collect()
            })
            .collect();

        Some(Matrix(result))
    }

}

impl<T: Clone> Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
       if self.0.is_empty() {
        return 0;
       }
       self.0[0].len()
	}


	pub fn col(&self, n: usize) -> Vec<T> {
        if n >= self.number_of_cols() {
            return vec![];
        }
        self.0.iter().map(|row| row[n].clone()).collect()
	}
}