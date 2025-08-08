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
    input.chars().map(|c| rotate_char(c, key)).collect::<String>()
}

fn rotate_char(c: char, key: i8) -> char {
    if c.is_ascii_alphabetic() {
        let base = if c.is_ascii_lowercase() { b'a' } else { b'A' } as i16;
        let offset = (c as i16 - base + key as i16) % 26;
        let rotated: u8 = if offset >= 0 { base + offset } else { base + offset + 26 } as u8;
        rotated as char
    } else {
        c // Non-alphabetic characters remain unchanged
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        let result = rotate("a", 26);
        let result2 =  rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5);
        let result3 = rotate("Testing", -14);
        let result4 = rotate("a", -1);
        assert_eq!(result, "a");
        assert_eq!(result2, "Ryg aesmuvi nkpd tewzsxq jolbkc foh");
        assert_eq!(result3, "Fqefuzs");
        assert_eq!(result4, "z");
    }

    #[test]
    fn it_works_2() {
        let result = rotate("m", 0);
        assert_eq!(result, "m");
    }

    #[test]
    fn test_3() {
        let result = rotate("MISS", 5);
        assert_eq!(result, "RNXX");
    }

    #[test]
    fn test_4() {
        let result = rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13);
        assert_eq!(result, "The five boxing wizards jump quickly.");
    }
}
