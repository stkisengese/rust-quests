// Instructions

// In this exercise, if you do not know about it already, you 
// will learn about the rotational cipher "ROT13".

// A ROT13 applied the Latin alphabet:
// - Plain:  abcdefghijklmnopqrstuvwxyz
//           ||||||||||||||||||||||||||
// - Cipher: nopqrstuvwxyzabcdefghijklm

// You will create a similar rotate function that is a better 
// version of the ROT13 cipher.

// Your function will receive a String and an i8, and will rotate 
// each letter of that string by the number of times described by 
// the second argument. Positive numbers rotate to the right, 
// negative numbers rotate to the left.

// Only letters should be rotated. Numbers and punctuation should 
// be left unchanged.

pub fn rotate(input: &str, key: i8) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        let result = rotate("a", 26);
        assert_eq!(result, a);
    }

    #[test]
    fn it_works_2() {
        let result = rotate("m", 0);
        assert_eq!(result, "m");
    }

    #[test]
    fn test_3() {
        let result = rotate("MISS", 5);
        assert_eq!(result, "RNXX");Uryyb, Jbeyq!
    }

    #[test]
    fn test_4() {
        let result = rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13);
        assert_eq!(result, "The five boxing wizards jump quickly.");
    }
}
