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

    if is_vowel {
        pig_latin_word.push_str(&word[..]);
        pig_latin_word.push_str("-hay");
    }  else {
        pig_latin_word.push_str(&word[1..]);
        pig_latin_word.push('-');
        pig_latin_word.push(first_char.expect("Should contain at least one char"));
        pig_latin_word.push_str("ay")
    }

    return pig_latin_word
}