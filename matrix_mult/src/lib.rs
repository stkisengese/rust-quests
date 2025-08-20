// pub use lalgebra_scalar::Scalar;
pub use std::ops::{Mul, Add, Sub, Div};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

// impl<T: Scalar<Item = T>> Mul for Matrix<T> {
//     type Output = Option<Self>;

//     fn mul(self, other: Self) -> Self::Output {
//         if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
//             return None;
//         }

//         let result: Vec<Vec<T>> = self.0.iter()
//             .zip(other.0.iter())
//             .map(|(row_a, row_b)| {
//                 row_a.iter()
//                     .zip(row_b.iter())
//                     .map(|(&a, &b)| a * b)
//                     .collect()
//             })
//             .collect();

//         Some(Matrix(result))
//     }

// }


impl<T> Mul for Matrix<T> where T: Clone + std::ops::Mul<Output = T> + std::iter::Sum<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut new_row = Vec::new();
            for j in 0..other.number_of_cols() {
                let sum = self.row(i)
                    .iter()
                    .zip(other.col(j).iter())
                    .map(|(a, b)| a.clone() * b.clone())
                    .sum();
                new_row.push(sum);
            }
            result.push(new_row);
        }
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

	pub fn number_of_rows(&self) -> usize {
         self.0.len()
	}

	pub fn row(&self, n: usize) -> Vec<T> {
        if n >= self.number_of_rows() {
            return vec![];
        }
        self.0[n].clone()
	}

	pub fn col(&self, n: usize) -> Vec<T> {
        if n >= self.number_of_cols() {
            return vec![];
        }
        self.0.iter().map(|row| row[n].clone()).collect()
	}
}