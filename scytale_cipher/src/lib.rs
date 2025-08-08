// Instructions

// Create a function which creates a scytale cipher (also known as spartan cipher).

// In practice, it is represented by a strip wrapped around a cylinder. The message 
// is written across the loops of the strip (not along the strip). The message becomes 
// coded if the radius of the cylinder changes, or the strip is removed from the cylinder.

// Your function should recreate the scytale cipher, so that the &str represents the 
// message, and the usize represents the number of times the strip is wrapped around the cylinder.
// Example

// size 6: "scytale Code" -> "sec yCtoadle"
// --------------------------------
//   |s|  |c|  |y|  |t|  |a|  |l|
//   |e|  | |  |C|  |o|  |d|  |e|
// --------------------------------

// size 8: "scytale Code" -> "sCcoydtea l e"
// ------------------------------------------
//   |s|  |c|  |y|  |t|  |a|  |l|  |e|  | |
//   |C|  |o|  |d|  |e|  | |  | |  | |  | |
// ------------------------------------------

pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 || message.is_empty() {
        return String::new();
    }

    let message: Vec<char> = message.chars().collect();
    let len = message.len();
    let cols = (len + i - 1) / i; // Calculate the number of columns needed
    let mut result = String::with_capacity(len);

    for col in 0..cols {
        for row in 0..i {
            let index = row * cols + col;
            if index < len {
                result.push(message[index]);
            } else {
                result.push(' '); // Fill with space if index is out of bounds
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = scytale_cipher("scytale Code", 6);
        let result2 = scytale_cipher("scytale Code", 8);
        assert_eq!(result, "sec yCtoadle");
        assert_eq!(result2, "sCcoydtea l e");
    }
}
