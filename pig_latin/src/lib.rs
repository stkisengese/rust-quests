// Instructions

// Create a function which transforms the string passed as an argument into Pig Latin:

//     If a word begins with a vowel, just add "ay" to the end.

//     If it begins with a consonant, then we take all consonants before the first vowel, 
//     move them to the end of the word, and then add "ay" at the end.

//     If a word starts with a consonant followed by "qu", move these three characters to 
//     the end of the word, and then add an "ay" at the end.

//     Only the latin vowels will be considered as vowels (aeiou).


pub fn pig_latin(text: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
         let result = pig_latin(&String::from("igloo"));
        let result2 = pig_latin(&String::from("apple"));
        let result3= pig_latin(&String::from("hello"));
        let result4 = pig_latin(&String::from("square"));
        let result5 = pig_latin(&String::from("xenon"));
        let result6 = pig_latin(&String::from("chair"));
        let result7 = pig_latin(&String::from("queen"));
    
        assert_eq!(result, "iglooay");
        assert_eq!(result2, "appleay");
        assert_eq!(result3, "ellohay");
        assert_eq!(result4, "aresquay");
        assert_eq!(result5, "enonxay");
        assert_eq!(result6, "airchay");
        assert_eq!(result7, "ueenqay");
    }
}
