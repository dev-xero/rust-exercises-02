pub fn convert(word: &str) -> String {
    if word.len() == 0 {
        panic!("A word to convert must be supplied");
    }

    let mut pig_latin_word = String::with_capacity(word.len() + 3);
    let first_char = word.chars().next().unwrap();
    let is_vowel = "aeiouAEIOU".contains(first_char);

    if is_vowel {
        pig_latin_word.push_str(word);
        pig_latin_word.push_str("-hay");
    }   else {
        let rest_of_word = &word[1..];
        pig_latin_word.push_str(rest_of_word);
        pig_latin_word.push('-');
        pig_latin_word.push(first_char.to_ascii_lowercase());
        pig_latin_word.push_str("ay");
    }

    return pig_latin_word;
}