pub use lalgebra_scalar::Scalar;
use std::ops::Add;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
  	if self.0.len() != other.0.len() { return None; }

	let result: Vec<T> = self.0.iter()
		.zip(other.0.iter())
		.map(|(&a, &b)| a + b)
		.collect();

	return Some(Vector(result));
    }
	
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())    	
    }
    
    pub fn dot(&self, other: &Self) -> Option<T> {
	if self.0.len() != other.0.len() { return None; }
	if self.0.is_empty() {return None; }

	let mut sum = self.0[0] * other.0[0];
	for i in 1..self.0.len() {
	  sum = sum + (self.0[i] * other.0[i]);
	}
	return Some(sum);	
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
	let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        assert_eq!(vector_1.dot(&vector_2), Some(3));
	assert_eq!((vector_1 + vector_2), Some(Vector(vec![5, 1, -6])));
    }
}
