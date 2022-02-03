mod logic;
mod words;

use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main()
{
    let mut won: bool = false;

    print!("Enter the word length: ");
    if stdout().flush().is_err()
    {
        eprintln!("Could not flush STDIN!");
        exit(3);
    }

    let mut word_len_str = String::new();
    if stdin().read_line(&mut word_len_str).is_err()
    {
        eprintln!("Could not read STDIN!");
        exit(4);
    }

    let word_len: usize = word_len_str
        .trim()
        .parse()
        .unwrap_or(5);

    let secret_word = words::get_rand_word(word_len);
    // println!("{secret_word}");

    let mut remaining_tries = 5;
    while remaining_tries != 0
    {
        print!("\nEnter your guess: ");
        if stdout().flush().is_err()
        {
            eprintln!("Could not flush STDIN!");
            exit(3);
        }

        let mut guess = String::new();
        if stdin().read_line(&mut guess).is_err()
        {
            eprintln!("Could not read STDIN!");
            exit(4);
        }

        guess = guess.trim().to_string();
        if guess.len() != word_len
        {
            println!("Guess must be {word_len} characters.");
            continue;
        }

        if !words::word_is_valid(&guess)
        {
            println!("Please enter an actual word.");
            continue;
        }

        let validity = logic::get_letter_validity(&guess, &secret_word, &word_len);

        for i in validity
        {
            print!("{}", i);
        }

        println!("");

        if logic::guess_is_correct(&guess, &secret_word) 
        { 
            won = true;
            break;
        }

        remaining_tries -= 1;
        println!("You have {remaining_tries} guesses left.");
    }

    if won == false 
    {
        println!("The word was {secret_word}");
    }
}