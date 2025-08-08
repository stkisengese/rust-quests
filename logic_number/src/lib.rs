// Instructions

// In this exercise, the logic for a sequence of numbers will be tested. You will 
// have to create a function which will return true if the number is the sum of its
// own digits, where each digit is first raised to the power of the number of digits.

// Examples:

//     9 returns true, because 9 = 9^1 = 9

//     10 returns false, because 10 != 1^2 + 0^2 = 1

//     153 returns true, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153

//     154 returns false, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

pub fn number_logic(num: u32) -> bool {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = number_logic(9);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_with_false() {
        let result = number_logic(10);
        assert_eq!(result, false);  
    }

    #[test]
    fn it_works_with_true() {
        let result = number_logic(153);
        assert_eq!(result, true);   
    }   

    #[test]
    fn it_works_with_false_again() {
        let result = number_logic(154);
        assert_eq!(result, false);
    }
}
