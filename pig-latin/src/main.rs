// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word
// and ay is added, so first becomes irst-fay. Words that start with
// a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

use std::io::{self, Write};

fn to_pig_latin(word: &str) -> String {
    match word.chars().next().expect("Cannot convert empty string!") {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            format!("{}-hay", word)
        }
        _ => {
            let (first_letter, rest) = word.split_at(1);
            format!("{}-{}ay", rest, first_letter)
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        println!("Pig-latin translator. Input your text:");
        io::stdout().flush()?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;

        let input = s.trim();
        if input.is_empty() {
            println!("Please enter a valid string!");
            continue;
        }

        let pig_latin_words: Vec<String> = input.split_whitespace().map(to_pig_latin).collect();
        println!("{}", pig_latin_words.join(" "));
    }
}
