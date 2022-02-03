use colored::{ColoredString, Colorize};

pub fn get_letter_validity(guess: &String, word: &String, len: &usize) -> Vec<ColoredString>
{
    let mut output: Vec<ColoredString> = vec![];

    let guess_chars: Vec<char> = guess.chars().collect();
    let word_chars: Vec<char> = word.chars().collect();

    for i in 0..*len
    {
        if guess_chars[i] == word_chars[i]
        {
            output.push(guess_chars[i].to_string().green());
        }

        else if word_chars.contains(&guess_chars[i])
        {
            output.push(guess_chars[i].to_string().yellow());
        }

        else
        {
            output.push(guess_chars[i].to_string().red());
        }
    }

    output
}

pub fn guess_is_correct(guess: &String, word: &String) -> bool
{
    guess == word
}
