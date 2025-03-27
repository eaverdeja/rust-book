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

        let words: Vec<String> = s.trim().split(' ').map(|s| s.to_string()).collect();
        for mut word in words {
            let res = match word.chars().next().expect("Cannot convert empty string!") {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    word.push_str("-hay");
                    word
                }
                _ => {
                    let first_letter = word.remove(0);
                    let res = "-".to_string() + &first_letter.to_string() + "ay";
                    word.push_str(&res);
                    word
                }
            };
            print!("{res} ");
        }
        println!("");
    }
}
