use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let list: Vec<&str> = input_str.split(',').collect();
    let mut hs = HashSet::new();
    for i in 0..list.len() {
        hs.insert(list[i]);
    }
    hs.len()
}
