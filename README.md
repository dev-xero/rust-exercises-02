# Rust Exercises: 02
Exercises at the end of chapter 08 of the 'Rust Programming Language' book

## Solutions
1. Shift Cipher (Extra)  
The shift cipher is a simple cryptographic algorithm that operates on the principle of shifting the letters in a word by a specified number of times. It doesn't handle uppercase and non-Latin letters yet.
    ```rust
    // src/shift_cipher.rs
    
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
    
        // -- [rest of implementation in code] --
    }
    ```

2. Median & Mode  
The algorithms are implemented using their simple standard definitions.

    ```rust
    // src/stats.rs
    
    pub fn median(list: &Vec<i32>) -> i32 {
        let size = list.len();
        let is_odd = size % 2 != 0;
        let mid = size / 2;
        
        if size == 0 { panic!("Cannot find the median of an empty list"); }
    
        let mut sorted_list: Vec<i32> = list.clone();
        sorted_list.sort();
    
        if is_odd {
            sorted_list[mid]
        } else {
            (sorted_list[mid - 1] + sorted_list[mid]) / 2
        }
    }
    ```
    
    ```rust
    // src/stats.rs
    
    pub fn mode(list: &Vec<i32>) -> i32 {
        use std::collections::HashMap;
    
        let mut num_count: HashMap<i32, i32> = HashMap::new();
        let size = list.len();
    
        if size == 0 { panic!("Cannot find mode of an empty list"); }
    
        let mut mode = -1;
        let mut max_count = 0;
    
        for num in list {
            let count = num_count.entry(*num).or_insert(0);
            *count += 1;
    
            if *count > max_count {
                max_count = *count;
                mode = *num;
            }
        }
    
        return mode;
    }
    ```

3. Word to Pig Latin  
   "The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a         vowel have “hay” added to the end instead (“apple” becomes “apple-hay”)"

   ```rust
   // src/pig_latin.rs
   
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
   ```
