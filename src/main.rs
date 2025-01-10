use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

fn colorize_attempt(attempt: &str, solution: &str) {
    let mut solution_char_count = HashMap::new();
    for c in solution.chars() {
        *solution_char_count.entry(c).or_insert(0) += 1;
    }

    let mut result = vec![' '; attempt.len()];
    for (i, c) in attempt.chars().enumerate() {
        if c == solution.chars().nth(i).unwrap() {
            result[i] = 'G';
            *solution_char_count.get_mut(&c).unwrap() -= 1;
        }
    }

    for (i, c) in attempt.chars().enumerate() {
        if result[i] == ' ' {
            if solution.contains(c) && *solution_char_count.get(&c).unwrap() > 0 {
                result[i] = 'Y';
                *solution_char_count.get_mut(&c).unwrap() -= 1;
            } else {
                result[i] = 'X';
            }
        }
    }

    for (i, c) in attempt.chars().enumerate() {
        match result[i] {
            'G' => print!("\x1b[32m{}\x1b[0m", c),
            'Y' => print!("\x1b[33m{}\x1b[0m", c),
            'X' => print!("\x1b[90m{}\x1b[0m", c),
            _ => (),
        }
    }
    println!();
}

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
    let mut previous_attempt: Option<String> = None;
    let mut keyboard_status = HashMap::new();

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
            if let Some(prev) = &previous_attempt {
                print!("Previous attempt was: ");
                colorize_attempt(&prev, &solution);
            }
            continue;
        }
        if guess == solution {
            println!("Correct! The word was: {}", solution);
            return;
        }

        // Create a table to keep track of the number of repeated letters
        let mut solution_char_count = HashMap::new();
        for c in solution.chars() {
            *solution_char_count.entry(c).or_insert(0) += 1;
        }

        let mut guess_char_count = HashMap::new();
        for c in guess.chars() {
            *guess_char_count.entry(c).or_insert(0) += 1;
        }

        // First pass: mark correct positions (green)
        let mut result = vec![' '; guess.len()];
        for (i, c) in guess.chars().enumerate() {
            if c == solution.chars().nth(i).unwrap() {
                result[i] = 'G';
                *solution_char_count.get_mut(&c).unwrap() -= 1;
                *guess_char_count.get_mut(&c).unwrap() -= 1;
                keyboard_status.insert(c, 'G');
            }
        }

        // Second pass: mark present but incorrect positions (yellow) and absent positions (gray)
        for (i, c) in guess.chars().enumerate() {
            if result[i] == ' ' {
                if solution.contains(c) && *solution_char_count.get(&c).unwrap() > 0 {
                    result[i] = 'Y';
                    *solution_char_count.get_mut(&c).unwrap() -= 1;
                    if keyboard_status.get(&c) != Some(&'G') {
                        keyboard_status.insert(c, 'Y');
                    }
                } else {
                    result[i] = 'X';
                    if keyboard_status.get(&c) != Some(&'G') && keyboard_status.get(&c) != Some(&'Y') {
                        keyboard_status.insert(c, 'X');
                    }
                }
            }
        }

        // Print the result with colors
        for (i, c) in guess.chars().enumerate() {
            match result[i] {
                'G' => print!("\x1b[32m{}\x1b[0m", c),
                'Y' => print!("\x1b[33m{}\x1b[0m", c),
                'X' => print!("\x1b[90m{}\x1b[0m", c),
                _ => (),
            }
        }
        println!();

        previous_attempt = Some(guess.clone());
        attempts += 1;

        // Print the keyboard layout
        print_keyboard(&keyboard_status);
    }
    println!("Sorry, you're out of attempts. The solution was: {}", solution);
}

fn print_keyboard(keyboard_status: &HashMap<char, char>) {
    let keyboard = [
        "qwertyuiop",
        "asdfghjkl",
        "zxcvbnm"
    ];

    for row in keyboard.iter() {
        for c in row.chars() {
            match keyboard_status.get(&c) {
                Some('G') => print!("\x1b[32m{}\x1b[0m ", c),
                Some('Y') => print!("\x1b[33m{}\x1b[0m ", c),
                Some('X') => print!("\x1b[90m{}\x1b[0m ", c),
                _ => print!("{} ", c),
            }
        }
        println!();
    }
    println!();
}