use std::io;
fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    
    let mut attempts = 0;

    loop {
        println!("{}", riddle);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        attempts += 1;

        if input.trim() == answer {
            println!("Number of trials: {}", attempts);
            break;
        }
    }
}
