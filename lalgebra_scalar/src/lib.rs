use std::ops::{Add, Sub, Mul, Div};
use std::{ marker::Sized, fmt::Debug};

pub trait Scalar:
	Add<Output = Self> +
    	Sub<Output = Self> +
   	Mul<Output = Self> +
    	Div<Output = Self> +
    	Sized +
    	Copy +
	Clone +
	Eq +
	PartialEq +
	Debug +
    	'static				
{
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for u32 { 
        type Item = u32;
        fn zero() -> Self::Item { 0 }
        fn one() -> Self::Item { 1 }
}
impl Scalar for u64 { 
        type Item = u64;
        fn zero() -> Self::Item { 0 }
        fn one() -> Self::Item { 1 }
}
impl Scalar for i32 { 
        type Item = i32;
        fn zero() -> Self::Item { 0 }
        fn one() -> Self::Item { 1 }
}
impl Scalar for i64 { 
        type Item = i64;
        fn zero() -> Self::Item { 0 }
        fn one() -> Self::Item { 1 }
}
impl Scalar for f32 { 
        type Item =f32;
        fn zero() -> Self::Item { 0.0 }
        fn one() -> Self::Item { 1.0 }
}

impl Scalar for f64 { 
        type Item = f64;
        fn zero() -> Self::Item { 0.0 }
        fn one() -> Self::Item { 1.0 }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(f64::zero(), 0.0);
	assert_eq!(i32::zero(), 0);
	assert_eq!(f64::one(), 1.0);
	assert_eq!(i32::one(), 1);
    }
}
