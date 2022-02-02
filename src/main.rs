mod logic;
mod words;

use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main()
{
    let secret_word = words::get_rand_word();
    // println!("{secret_word}");

    let mut remaining_tries = 5;
    while remaining_tries != 0
    {
        print!("Enter your guess: ");
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
        if guess.len() != 5
        {
            println!("Guess must be 5 characters.");
            continue;
        }

        let validity = logic::get_letter_validity(&guess, &secret_word);

        for i in validity
        {
            print!("{}", i);
        }

        println!("");

        if logic::guess_is_correct(&guess, &secret_word) { break }

        remaining_tries -= 1;
    }

    println!("The word was {secret_word}");
}