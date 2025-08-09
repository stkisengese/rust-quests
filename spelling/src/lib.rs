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
/// 
const UNITS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty",
    "sixty", "seventy", "eighty", "ninety",
];  

pub fn spell(n: u32) -> String {
    match n {
        0..=19 => UNITS[n as usize].to_string(),
        20..=99 => {
            let tens = TENS[(n / 10) as usize];
            match n % 10 {
                0 => tens.to_string(),
                u => format!("{}-{}", tens, UNITS[u as usize]),
            }
        },
        100..=999 => {
            let hundreds = UNITS[(n / 100) as usize];
            match n % 100 {
                0 => format!("{} hundred", hundreds),
                u => format!("{} hundred {}", hundreds, spell(u)),
            }
        },
        1000..=999_999 => format!("{} thousand {}", spell(n/1000), spell(n % 1000)),
        1_000_000 => "one million".to_string(),
        _ => unreachable!(), // Since we only handle up to one million
    }
}

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
