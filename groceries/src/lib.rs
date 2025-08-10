// Instructions

// Create a function named insert, that inserts a new element at the end of the Vec.

// Create another function named at_index that returns the value found at the index passed as an argument.

pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);

}

pub fn at_index(slice: &[String], index: usize) -> &str {
    slice.get(index)
        .expect("Index out of bounds")
        .as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
        "milk".to_string(),
        ];
        insert(&mut groceries, String::from("nuts"));
        assert_eq!(groceries[5], "nuts");
    }

    #[test]
    fn at_index_test() {
        let vec = vec!["apple".to_string(), "banana".to_string()];
        let result = at_index(&vec, 1);
        assert_eq!(result, "banana");
    }
}
