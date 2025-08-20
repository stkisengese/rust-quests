pub fn delete_prefix(prefix: &str, s: &str) -> Option<&str> {
    prefix.strip_prefix()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = delete_prefix("ab", "abcdefghijklmnop");
	let result2 = delete_prefix("x", "abcdefghijklmnop");
 
       assert_eq!(result, Some("cdefghijklmnop"));
 	assert_eq!(result2, None);  
 }
}
