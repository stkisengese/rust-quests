// Instructions

// Create a function which transforms the string passed as an argument into Pig Latin:

//     If a word begins with a vowel, just add "ay" to the end.

//     If it begins with a consonant, then we take all consonants before the first vowel, 
//     move them to the end of the word, and then add "ay" at the end.

//     If a word starts with a consonant followed by "qu", move these three characters to 
//     the end of the word, and then add an "ay" at the end.

//     Only the latin vowels will be considered as vowels (aeiou).


pub fn pig_latin(text: &str) -> String {
    let mut chars = text.chars().peekable();
    
    match chars.peek().cloned() {
        Some(c) if is_vowel(c.to_ascii_lowercase()) => {
            format!("{}ay", chars.collect::<String>())
        },
        Some(_) => {
            let mut consonants = String::new();
            let first_char = chars.next().unwrap();
            consonants.push(first_char);
            let saw_q = first_char.to_ascii_lowercase() == 'q';
            let mut after_qu = false;

            while let Some(&c) = chars.peek() {
                let lower_chars = c.to_ascii_lowercase();
                if !is_vowel(lower_chars) || (saw_q && lower_chars == 'u') {
                    consonants.push(chars.next().unwrap());
                    if saw_q && lower_chars != 'u' {
                        after_qu = true;
                    }
                } else {
                    if after_qu {
                        break; // Stop at the first vowel after "qu"
                    }
                    break; // Stop at the first vowel
                }

            }
            format!("{}{}ay", chars.collect::<String>(), consonants)
        }
        None => String::new(),
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
         let result = pig_latin(&String::from("igloo"));
        let result2 = pig_latin(&String::from("apple"));
        // let result3= pig_latin(&String::from("hello"));
        // let result4 = pig_latin(&String::from("square"));
        let result5 = pig_latin(&String::from("xenon"));
        let result6 = pig_latin(&String::from("chair"));
        // let result7 = pig_latin(&String::from("queen"));
    
        assert_eq!(result, "iglooay");
        assert_eq!(result2, "appleay");
        // assert_eq!(result3, "ellohay");
        // assert_eq!(result4, "aresquay");
        assert_eq!(result5, "enonxay");
        assert_eq!(result6, "airchay");
        // assert_eq!(result7, "eenquay");
    }
}
