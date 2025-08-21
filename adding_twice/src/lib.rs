pub fn add_curry(t: i32) -> impl Fn(i32) -> i32 {
    move |x| t + x
}

pub fn twice(f: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
 	let add5 = add_curry(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-5), 0);        
    }
}
