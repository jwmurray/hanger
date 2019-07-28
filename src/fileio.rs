use rand::Rng;
use std::fs;

pub fn get_word() -> String {
    let filename = "words.txt";
    let file_string =
        fs::read_to_string(filename).expect(&format!("Unable to read file {}", filename));
    let word_list: Vec<&str> = file_string.split_whitespace().collect();

    let secret_word = word_list[rand::thread_rng().gen_range(0, word_list.len())];
    String::from(secret_word)
}
