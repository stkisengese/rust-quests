#[derive(Debug)]
pub struct Person{
	pub name: &str,
	pub age: u8,
}

impl Person {
	pub fn new(name: &str) -> Person {
		Person{ name, age: 0 }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let person = Person::new("Leo");
        assert_eq!(person, Person{name: "Leo", age: 0 });
    }
}
