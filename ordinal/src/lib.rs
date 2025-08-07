// Instructions

// Complete the function num_to_ordinal. 
// It returns the ordinal number for a given cardinal number.

pub fn num_to_ordinal(x: u32) -> String {
    let suffix = match (x%10, x%100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{}{}", x, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = num_to_ordinal(47);
        assert_eq!(result, "47th");
    }

    #[test]
    fn it_works_for_22nd() {
        let result = num_to_ordinal(22);
        assert_eq!(result, "22nd");
    }
    #[test]
    fn it_works_for_12th() {    
        let result = num_to_ordinal(12);
        assert_eq!(result, "12th");
    }
}
