pub fn first_fifty_even_square() -> Vec<i32> {
	(1..=50).map(|x| (x as i32 *2).pow(2)).collect()
}

