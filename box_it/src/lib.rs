// Instructions

// Create the following functions:

//     parse_into_boxed: which accepts a string of numbers separated by spaces. 
//     If a number has a k as a suffix it should be multiplied by 1000. The function 
//     parses these numbers and boxes them into a vector of Box<u32>.

//     into_unboxed: which accepts the value returned from parse_into_boxed 
//     and unboxes each element into another vector.

pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
   s.split_whitespace()
        .map(|num_str| {
            if num_str.ends_with('k') {
                let num = num_str.trim_end_matches('k').parse::<f32>().unwrap() * 1000.0;
                Box::new(num as u32)
            } else {
                Box::new(num_str.parse::<u32>().unwrap())
            }
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "5.5k 8.9k 32".to_owned();
        let boxed = parse_into_boxed(s);
        let result = into_unboxed(boxed);
        assert_eq!(result, [5500, 8900, 32]);
       
    }
}
