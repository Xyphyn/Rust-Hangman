use rand::seq::IteratorRandom;
use std::{fs, io, process};

fn main() {
    let words = fs::read_to_string("./words.txt");
    if words.is_err() {
        println!("Create a file named words.txt with a word on each line.");
        process::exit(1);
    }

    let binding = words.unwrap();
    let words = binding.lines();

    let word = words.choose(&mut rand::thread_rng()).unwrap();
    let mut guesses: Vec<char> = Vec::new();

    let response = guess(&guesses, word);
    let mut misses = 0;
    println!("{}", response);

    loop {
        println!("Enter a letter... [Misses: {misses}]");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let choice = input.trim().parse::<char>();

        if choice.is_err() {
            continue;
        }

        let choice = choice.unwrap();

        if !word.contains(choice) {
            misses += 1
        }

        if misses >= 6 {
            println!("You lose!");
            break
        }

        guesses.push(choice);

        let response = guess(&guesses, word);

        println!("{}", response);

        if !response.contains('_') {
            println!("You win!");
            break;
        }
    }
}

fn guess(characters: &[char], word: &str) -> String {
    word.chars()
        .map(|c| if characters.contains(&c) { c } else { '_' })
        .collect()
}
