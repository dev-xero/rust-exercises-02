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