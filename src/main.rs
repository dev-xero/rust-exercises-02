mod stats;

fn test_median() {
    println!("\nMedian of a set of numbers");
    println!("---");

    let even_numbered_list = vec![18, 45, 72, 33, 91, 14, 67, 29, 56, 83];
    let median_of_even = stats::median(&even_numbered_list);
    
    println!("EVEN: {:?}", even_numbered_list);
    println!("The median is: {}\n", median_of_even);

    let odd_numbered_list = vec![17, 42, 88, 29, 55, 71, 38, 93, 62];
    let median_of_odd = stats::median(&odd_numbered_list);
    
    println!("ODD: {:?}", odd_numbered_list);
    println!("The median is: {}", median_of_odd);
}

fn test_mode() {
    println!("\nMode of a set of numbers");
    println!("---");

    let test_list = vec![3, 2, 5, 3, 8, 7, 3, 4, 3];
    let mode = stats::mode(&test_list);
    
    println!("LIST: {:?}", test_list);
    println!("The mode is: {}", mode);
}

fn main() {
    println!("Exercise 02");

    test_median();
    test_mode();
}
