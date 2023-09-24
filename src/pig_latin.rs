pub fn convert(word: &str) -> String {
    if word.len() == 0 {
        panic!("A word to convert must be supplied");
    }


    let mut pig_latin_word = String::new();
    let mut is_vowel = false;
    let first_char = word.chars().nth(0);
    let vowels = "aeiou";
    
    if let Some(first) = first_char {
        if vowels.contains(first) {
            is_vowel = true;
        }
    }

    let new_first_char = word
        .chars()
        .nth(1)
        .expect("Should contain at least one char")
        .to_ascii_uppercase();

    pig_latin_word.push(new_first_char);
    pig_latin_word.push_str(&word[2..]);

    let first_char = first_char
        .expect("Should contain at least one char")
        .to_ascii_lowercase();

    if is_vowel {
        pig_latin_word.push_str("-hay");
    }  else {
        pig_latin_word.push('-');
        pig_latin_word.push(first_char);
        pig_latin_word.push_str("ay")
    }

    return pig_latin_word
}