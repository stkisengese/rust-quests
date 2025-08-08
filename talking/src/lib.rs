
// Instructions

// Build the function talking which will allow you to talk with your computer.

// Its answers will be created by you following the rules below.

//     It answers "There is no need to yell, calm down!" if you yell at it. For example "LEAVE ME ALONE!". Yelling is when all the letters are capital letters.
//     It answers "Sure." if you ask it something without yelling. For example "Is everything ok with you?".
//     It answers "Quiet, I am thinking!" if you yell a question at it. FOr example: "HOW ARE YOU?".
//     It says "Just say something!" if you address it without actually saying anything.
//     It answers "Interesting" to anything else.

pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    if is_yelling(text) {
        if is_question(text) {
            return "Quiet, I am thinking!";
        }
        return "There is no need to yell, calm down!";
    } else if is_question(text) {
        return "Sure.";
    } else {
        return "Interesting";
    }
}

fn  is_question(text: &str) -> bool {
    text.trim().ends_with('?')
}

fn is_yelling(text: &str) -> bool {
    text.chars().any(|c| c.is_alphabetic()) && 
    text.chars().all(|c| c.is_uppercase() || !c.is_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =  talking("JUST DO IT!");
        assert_eq!(result, "There is no need to yell, calm down!");
    }

    #[test]
    fn it_works_with_question() {
        let result = talking("Hello how are you?");
        assert_eq!(result, "Sure.");
    }

    #[test]
    fn it_works_with_yelling_question() {
        let result = talking("WHAT IS YOUR NAME?");
        assert_eq!(result, "Quiet, I am thinking!");
    }

    #[test]
    fn it_works_with_empty_string() {
        let result = talking("");
        assert_eq!(result, "Just say something!");  
    }

    #[test]
    fn it_works_with_other_text() {
        let result = talking("I love programming.");
        assert_eq!(result, "Interesting");  
    }
}
