use crate::Letter;
// use std::collections::HashMap;

pub fn print_status(turns_left: usize, display_letters: &Vec<Letter>) {

    // let char_map = HashMap::new();
    let mut output_word = String::from("");

    for letter in display_letters {
        let print_char = match letter.revealed {
            false => '_',
            true => letter.character,
        };

        output_word.push(print_char);
        output_word.push(' ');
    }

    println!(
        "Status:\n\tturns_left: {}, word: {}",
        turns_left, output_word
    );
}
