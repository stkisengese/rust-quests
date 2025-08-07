// Instructions

// Create a function named is_pangram which returns true if the given string is a pangram.

// A pangram is a sentence which uses every letter of the alphabet at least once.

// Example: "The quick brown fox jumps over the lazy dog."

pub fn is_pangram(s: &str) -> bool {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>()
        .len() == 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =  is_pangram("the quick brown fox jumps over the lazy dog!");
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_with_uppercase() {
        let result = is_pangram("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG!");
        assert_eq!(result, true);
    }

    #[test]
    fn it_fails_with_missing_letters() {
        let result = is_pangram("the quick brown fox jumps over the lazy");
        assert_eq!(result, false);
    }
}
