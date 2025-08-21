pub fn add_curry(t: i32) -> impl Fn(i32) -> i32 {
    move |x| t + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let add10 = add_curry(-10);
    	let add20 = add_curry(2066);
    	let add30 = add_curry(300000);
	assert_eq!(add10(5), -5);
    	assert_eq!(add20(195), 2261);
    	assert_eq!(add30(5696), 305696);       
    }
}
