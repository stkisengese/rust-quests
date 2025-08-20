#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

impl<'a> Person<'a> {
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
