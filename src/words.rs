use std::fs::File;
use std::io::Read;
use std::process::exit;

use rand::seq::SliceRandom;

pub fn get_words() -> String
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

    word_list
}

pub fn get_rand_word() -> String
{
    let mut word_list = get_words();
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
