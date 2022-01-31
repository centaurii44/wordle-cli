use std::fs::File;
use std::io::{Read, Result};


fn main() -> Result<()> {
    let words_path = "words.txt";
    let mut words_file = File::open(words_path)?;

    let mut word_list = String::new();
    words_file.read_to_string(&mut word_list)?;

    word_list = word_list.replace("\r", "");
    let word_vec: Vec<&str> = word_list.split("\n").collect();

    Ok(())
}