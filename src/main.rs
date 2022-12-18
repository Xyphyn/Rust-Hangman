use std::io;

fn main() {
    let word = String::from("potato");
    let mut guesses: Vec<char> = Vec::new();

    loop {
        println!("Enter a letter...");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let choice = input.trim().parse::<char>();

        if choice.is_err() {
            continue;
        }

        let choice = choice.unwrap();

        guesses.push(choice);

        let response = guess(&guesses, &word);

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
