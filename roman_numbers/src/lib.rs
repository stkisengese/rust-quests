use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(_value: u32) -> Self {
        RomanDigit::Nulla
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 { return RomanNumber(vec![RomanDigit::Nulla]); }

        let mut result = Vec::new();

        while num > 0 {
            match num {
                n if n >= 1000 => { result.push(RomanDigit::M); num -= 1000; },
                n if n >= 900 => { result.push(RomanDigit::C); result.push(RomanDigit::M); num -=900; },
                n if n >= 500 => { result.push(RomanDigit::D); num -= 500; },
                n if n >= 400 => { result.push(RomanDigit::C); result.push(RomanDigit::D); num -= 400; },
                n if n >= 100 => { result.push(RomanDigit::C); num -= 100; },
                n if n >= 90 => { result.push(RomanDigit::X); result.push(RomanDigit::C); num -= 90; },
                n if n >= 50 => { result.push(RomanDigit::L); num -= 50; },
                n if n >= 40 => { result.push(RomanDigit::X); result.push(RomanDigit::L); num -= 40; },
                n if n >= 10 => { result.push(RomanDigit::X); num -= 10; },
                n if n >= 9 => { result.push(RomanDigit::I); result.push(RomanDigit::X); num -= 9; },
                n if n >= 5 => { result.push(RomanDigit::V); num -= 5; },
                n if n >= 4 => { result.push(RomanDigit::I); result.push(RomanDigit::V); num -= 4; },
                n if n >= 1 => { result.push(RomanDigit::I); num -= 1; },
                _ => break,
            }
        }
        return RomanNumber(result);
    }
}

impl RomanNumber {
    // Helper function to convert RomanNumber back to u32
    pub fn to_u32(&self) -> u32 {
        let mapping = [
            (M, 1000),
            (D, 500),
            (C, 100),
            (L, 50),
            (X, 10),
            (V, 5),
            (I, 1),
            (Nulla, 0),
        ];

        let mut total = 0;
        let mut prev_value = 0;

        for digit in self.0.iter().rev() {
            let value = mapping.iter().find(|(d, _)| *d == *digit).unwrap().1;

            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }

            prev_value = value;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(RomanNumber::from(0), RomanNumber([Nulla].to_vec()));
    }
}
