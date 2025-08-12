pub fn rev_str(input: &str) -> String {
    let mut result: String = String::new();
	for c in input.chars().rev() {
		result.push(c);
	}
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rev_str("!rac ecin a evah I");
        assert_eq!(result, "I have a nice car!");
    }
}
