// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word
// and ay is added, so first becomes irst-fay. Words that start with
// a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    loop {
        println!("Pig-latin translator. Input your text:");

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line!");

        let words: Vec<&str> = s.trim().split_whitespace().collect();

        if words.is_empty() {
            println!("Please enter a valid string!");
            continue;
        }

        for word in words {
            let res = match word.chars().next().expect("Cannot convert empty string!") {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    format!("{}-hay", word)
                }
                _ => {
                    let (first_letter, rest) = word.split_at(1);
                    format!("{}-{}ay", rest, first_letter)
                }
            };
            print!("{res} ");
        }
        println!("");
    }
}
