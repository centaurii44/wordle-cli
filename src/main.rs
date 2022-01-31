use std::fs::File;
use std::io::{Read, Result};


fn main()
{
    let secret_word = get_rand_word();
}

fn get_rand_word() -> Result<&'static str>
{
    let words_path = "words.txt";
    let mut words_file = File::open(words_path)?;

    let mut word_list = String::new();
    words_file.read_to_string(&mut word_list)?;

    word_list = word_list.replace("\r", "");

    let mut word_vec: Vec<&str> = vec![];
    for i in word_list.split_whitespace()
    {
        if i.len() == 5
        {
            word_vec.push(i);
        }
    }

    //println!("{word_vec:?}");


    Ok("random functionality coming later")
}