use roman_numbers::RomanNumber;

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value: u32 = self.to_u32();
        value += 1;
        *self = RomanNumber::from(value);
        Some(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut number = RomanNumber::from(15);
        assert_eq!(number, RomanNumber([X, V]));
        assert_eq!(number.next(), Some(RomanNumber([X, V, I])));
    }
}
