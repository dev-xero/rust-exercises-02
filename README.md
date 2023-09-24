# Rust Exercises: 02
Exercises at the end of chapter 08 of the 'Rust Programming Language' book

## Solutions
1. Median & Mode  
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

2. Word to Pig Latin  
   "The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a         vowel have “hay” added to the end instead (“apple” becomes “apple-hay”)"

   ```rust
   // src/pig_latin.rs
   
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
   ```
