mod logic;

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

use rand::seq::SliceRandom;

fn main()
{
    let secret_word = get_rand_word();
    println!("{secret_word}");

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
        println!("{:?}", logic::get_letter_validity(&guess, &secret_word));

        if logic::guess_is_correct(&guess, &secret_word) { break }

        remaining_tries -= 1;
    }
}

fn get_rand_word() -> String
{
    let words_path = "words.txt";
    let mut words_file = match File::open(words_path)
    {
        Ok(f) => f,
        Err(_) => 
        {
            eprintln!("Could not open words.txt!");
            exit(1);
        }
    };

    let mut word_list = String::new();
    if words_file.read_to_string(&mut word_list).is_err()
    {
        eprintln!("Could not read words.txt!");
        exit(2);
    }

    word_list = word_list.replace("\r", "");

    let mut word_vec: Vec<&str> = vec![];
    for i in word_list.split_whitespace()
    {
        if i.len() == 5
        {
            word_vec.push(i);
        }
    }

    let secret_word = word_vec.choose(&mut rand::thread_rng());
    secret_word.unwrap().to_string()
}