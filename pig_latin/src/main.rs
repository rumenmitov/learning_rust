mod utils;

use std::io;
use crate::utils::Letters;

fn main() {
    println!("Pig-Latin Translator!");
    let mut input = String::new();
    
    println!("Please input your text in English: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error! Couldn't read input!");

    input = input.to_ascii_lowercase();

    let first_letter = Letters::new(input.chars().nth(0));
    match first_letter {
        Letters::Vowel => println!("Translation: {}-hay", input.trim()),
        Letters::Consonants(first_letter) => {
            let mut translation = String::new();
            for letter in input.trim().chars() {
                if letter == first_letter {
                    continue;
                }
                translation.push(letter);
            }
            println!("Translation: {translation}-{first_letter}ay");
        },
        Letters::Err(err) => println!("{err}"),
    }
}
