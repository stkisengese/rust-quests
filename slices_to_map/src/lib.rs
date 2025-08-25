use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub fn slices_to_map<'a, T: Eq + Hash , U>(slice1: &'a [T], slice2: &'a [U]) -> HashMap<&'a T, &'a U> {
    slice1.iter().zip(slice2.iter()).collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
	let values = [1, 3, 23, 5, 2];
	let result = slices_to_map(&keys, &values);
        assert_eq!(result, {"James": 2, "Liam": 3, "Emma": 23, "Noah": 5, "Olivia": 1});
    }
}
