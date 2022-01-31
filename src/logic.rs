pub enum LetterPlacement
{
    CorrectSpot,
    CorrectLetter,
    Incorrect,
}

pub fn get_letter_validity(guess: &String, word: &String) -> Vec<LetterPlacement>
{
    let mut output: Vec<LetterPlacement> = vec![];

    let guess_chars: Vec<char> = guess.chars().collect();
    let word_chars: Vec<char> = word.chars().collect();

    for i in 0..5
    {
        if guess_chars[i] == word_chars[i]
        {
            output.push(LetterPlacement::CorrectSpot);
        }

        else if word_chars.contains(&guess_chars[i])
        {
                output.push(LetterPlacement::CorrectLetter);
        }

        else
        {
            output.push(LetterPlacement::Incorrect);
        }
    }

    output
}

pub fn guess_is_correct(guess: &String, word: &String) -> bool
{
    guess == word
}