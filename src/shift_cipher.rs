pub fn encrypt(word: &str, shift: usize) -> String {
    fn encrypt_char(letter: char, shift: usize) -> char {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        match alphabet.find(letter) {
            Some(word_index) => {
                let shift_index = (word_index + shift) % 26;
                let shifted_char = alphabet.chars().nth(shift_index).unwrap();

                return shifted_char;
            },
            None => {
                panic!("Invalid char '{}' encountered", letter);
            }
        }
    }

    fn encrypt_num(num: char, shift: usize) -> char {
        let i_num = num.to_digit(10);

        match i_num {
            Some(digit) => {
                let shifted_num = (digit + (shift as u32)) % 10;

                return char::from_digit(shifted_num, 10)
                    .expect("Failed to convert digit to char");
            },
            None => {
                panic!("Char '{}' cannot be converted to a digit", num);
            }
        }
    }

    if word.is_empty() {
        panic!("A word to convert is required");
    }

    let size = word.len();
    let mut encrypted_word = String::with_capacity(size);

    for ch in word.chars() {
        if ch.is_alphabetic() {
            encrypted_word.push(encrypt_char(ch, shift));
        } else if ch.is_numeric() {
            encrypted_word.push(encrypt_num(ch, shift));
        }
    }

    return encrypted_word;
}