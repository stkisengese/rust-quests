pub fn stars(n: u32) -> String {
    "*".repeat(2usize.pow(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = stars(4);
        assert_eq!(result, "****************");
    }
}
