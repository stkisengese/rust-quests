// Instructions

// Let's play a little.

// Create a function named score that given a &str, computes the score for that given string as a u64.

// Each letter has a value, you just have to sum the values of the letters in the given string.

// You will need these:
// Letter 	Value
// A, E, I, O, U, L, N, R, S, T 	1
// D, G 	2
// B, C, M, P 	3
// F, H, V, W, Y 	4
// K 	5
// J, X 	8
// Q, Z 	10

pub fn score(word: &str) -> u64 {
    let mut scores =[0u8; 126];
    for c in "aeioulnrst".chars() {
        scores[c as usize] = 1;
    }
    for c in "dg".chars() {
        scores[c as usize] = 2;
    }
    for c in "bcmp".chars() {
        scores[c as usize] = 3;
    }
    for c in "fhvwy".chars() {
        scores[c as usize] = 4;
    }
    scores['k' as usize] = 5;
    for c in "jx".chars() {
        scores[c as usize] = 8;
    }
    for c in "qz".chars() {
        scores[c as usize] = 10;
    }   

    word.chars()
        .map(|c| c.to_ascii_lowercase() as u8)
        .map(|c| scores[c as usize] as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = score("ThiS is A Test");
        assert_eq!(result, 14);
    }
}
