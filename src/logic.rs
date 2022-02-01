use colored::{ColoredString, Colorize};

/* This was used before colored letters were added. Just commenting it out for historical reasons
 * (if that makes sense)
#[derive(Debug)]
pub enum LetterPlacement
{
    CorrectSpot,
    CorrectLetter,
    Incorrect,
}
*/
pub fn get_letter_validity(guess: &String, word: &String) -> Vec<ColoredString>
{
    let mut output: Vec<ColoredString> = vec![];

    let guess_chars: Vec<char> = guess.chars().collect();
    let word_chars: Vec<char> = word.chars().collect();

    for i in 0..5
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
