// Splitting function return the vector of chars.
//
// #Argument
// String- String of type &str.
//
// #Return
// Store-A vector of chars.

pub fn splitting(string: &str) -> Vec<char> {
    let mut store: Vec<char> = Vec::new();
    for chars in string.chars() {
        store.push(chars)
    }
    store
}

// pattern_value function return the starting index of matched position.
//
// Return.
// Index- match position of type usize or 0.

pub fn pattern_value() -> usize {
    let word = String::from("pankaj chaudhary");
    let pattern = String::from("cha");
    let first = splitting(&word);
    let second = splitting(&pattern);
    let mut index = 0;
    while index <= word.len() - pattern.len() {
        let mut pattern_index = 0;
        while pattern_index < pattern.len() {
            if first[index + pattern_index] != second[pattern_index] {
                break;
            }
            pattern_index += 1;
        }
        if pattern_index == pattern.len() {
            return index;
        }
        index += 1;
    }
    0
}
