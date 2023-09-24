# Rust Exercises: 02
Exercises at the end of chapter 08 of the 'Rust Programming Language' book

## Solutions
1. Median & Mode  
The algorithms are implemented using their simple standard definitions.

    ```rust
    // src/stats/mod.rs
    
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
    // src/stats/mod.rs
    
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
