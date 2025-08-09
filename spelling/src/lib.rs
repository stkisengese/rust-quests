/// Instructions

/// In this exercise, you'll create the function spell that will spell a generated number.

/// Here are some examples of what your function should return:

///     1 -> "one"
///     14 -> "fourteen".
///     96 -> "ninety-six"
///    100 -> "one hundred".
///     101 -> "one hundred one"
///     348 -> "three hundred forty-eight"
///     1002 -> "one thousand two".
///     1000000 -> "one million"

///     Only positive numbers will be tested, up to "one million".

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(spell(348), "three hundred forty-eight");
        assert_eq!(spell(1000000), "one million");
        assert_eq!(spell(101), "one hundred one");
        assert_eq!(spell(999999), "nine hundred ninety-nine thousand nine hundred ninety-nine");
        assert_eq!(spell(0), "zero");
    }
}
