use crate::Letter;

pub fn create_letters(word: &str) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();
    for ch in word.chars() {
        letters.push(Letter {
            character: ch,
            revealed: false,
        });
    }
    return letters;
}
