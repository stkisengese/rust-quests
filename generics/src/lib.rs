pub fn identity<T>(v: T) -> T {
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_test() {
        assert_eq!(identity(3), 3);
    }
}
