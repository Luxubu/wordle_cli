use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

fn main() {
    let file = File::open("valid-wordle-words.txt").expect("Could not open wordle.txt");
    let reader = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|w| w.trim().to_lowercase())
        .filter(|w| w.len() == 5)
        .collect();

    let solution = {
        let mut rng = thread_rng();
        words.choose(&mut rng).unwrap().clone()
    };

    println!("Welcome to Wordle CLI! You have 6 tries.");
    let mut attempts = 0;
    while attempts < 6 {
        print!("Attempt {} - Enter a 5-letter word: ", attempts + 1);
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read input");
        let guess = guess.trim().to_lowercase();

        if guess.len() != 5 {
            println!("Input must be 5 letters.");
            continue;
        }
        if !words.contains(&guess) {
            println!("Not a Word.");
            continue;
        }
        if guess == solution {
            println!("Correct! The word was: {}", solution);
            return;
        }

        for (i, c) in guess.chars().enumerate() {
            if c == solution.chars().nth(i).unwrap() {
                print!("\x1b[32m{}\x1b[0m", c);
            } else if solution.contains(c) {
                print!("\x1b[33m{}\x1b[0m", c);
            } else {
                print!("\x1b[90m{}\x1b[0m", c);
            }
        }
        println!();
        attempts += 1;
    }
    println!("Sorry, you're out of attempts. The solution was: {}", solution);
}