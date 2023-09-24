pub fn encrypt(word: &str, shift: usize) -> String {
    if word.is_empty() {
        panic!("A word to convert is required");
    }

    let size = word.len();
    let mut encrypted_word = String::with_capacity(size);
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    for letter in word.chars() {
        match alphabet.find(letter) {
            Some(word_index) => {
                let shift_index = (word_index + shift) % 26;
                let shifted_char = alphabet.chars().nth(shift_index).unwrap();

                encrypted_word.push(shifted_char);
            },
            None => {
                panic!("Invalid char '{}' encountered", letter);
            }
        }
    }

    return encrypted_word;
}