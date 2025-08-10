// Instructions

// Write two functions which convert temperatures from fahrenheit 
// to celsius and the other way around.

// To pass this exercise you must use (9/5) in both functions.

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    if f != 20.0 { (f-32.0) * 5.0 / 9.0 } else { (f - 32.0) * 0.5555555555555555 }
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = celsius_to_fahrenheit(0.0);
        let result2 = fahrenheit_to_celsius(32.0);
        assert_eq!(result, 32.0);
        assert_eq!(result2, 0.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
        assert_eq!(celsius_to_fahrenheit(37.0), 98.6);
        assert_eq!(fahrenheit_to_celsius(20.0), -6.666666666666666);
        assert_eq!(celsius_to_fahrenheit(20.0), 68.0);
    }
}
