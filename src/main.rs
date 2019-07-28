mod fileio;
mod letters;
mod status;

use letters::*;
use status::*;

use std::io;

enum Game_Status {
    Underway,
    Won,
    Lost,
}

pub struct Letter {
    character: char,
    revealed: bool,
}

fn main() {
    let mut turns_left: usize = 5;
    let secret_word = fileio::get_word();
    let mut display_letters: Vec<Letter> = create_letters(&secret_word);

    println!("Secret word: {}", secret_word);

    loop {
        print_status(turns_left, &display_letters);
        println!("\nPlease guess a letter:  ");

        let new_char = read_user_input();
        // let new_char = 'z';

        if new_char == '*' {
            println!("Asterisk entered, exiting....");
            return;
        }

        if !update_letters(new_char, &mut display_letters) {
            turns_left -= 1;
        }

        match check_progress(turns_left, &display_letters) {
            Game_Status::Won => {
                println!("You Won!  The word was {}", secret_word);
                return;
            }
            Game_Status::Lost => {
                println!(
                    "So sorry, you ran out of tries.  The word was {}",
                    secret_word
                );
                return;
            }
            Game_Status::Underway => {}
        }
    }
}

fn read_user_input() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => return c,
            None => return '*',
        },
        Err(_) => return '*',
    }
}

fn update_letters(new_char: char, letters: &mut Vec<Letter>) -> bool {
    println!("You entered a {}", new_char);
    let mut guessed_correctly = false;
    for letter in letters {
        match letter.character == new_char {
            true => {
                if letter.revealed {
                    println!("You already guessed a {}.", new_char);
                }
                letter.revealed = true;
                guessed_correctly = true;
            }
            false => {}
        }
    }
    if !guessed_correctly {
        println!("...which was not in the secret word.");
    }
    guessed_correctly
}

fn check_progress(turns_left: usize, letters: &Vec<Letter>) -> Game_Status {
    fn letters_all_revealed(letters: &Vec<Letter>) -> bool {
        for ch in letters.iter() {
            if ch.revealed == false {
                return false;
            }
        }
        return true;
    }

    if letters_all_revealed(letters) {
        return Game_Status::Won;
    }
    if turns_left > 0 {
        return Game_Status::Underway;
    }
    return Game_Status::Lost;
}
