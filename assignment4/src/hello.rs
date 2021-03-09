use log::{debug, error, info, log_enabled, Level};
// Function Checking the repeating character in a word.
//
// #Argument
// word-a reference argument of type str.
//
// Return
// Return the repeating character in the word.
pub fn repeat_finder(word: &str) -> String {
    let mut my_vec: Vec<char> = word.chars().collect();
    let mut index = 0;
    let mut result = String::new();
    while index < word.len() {
        let mut counter = 1;
        let mut next_index = index + 1;
        while next_index < word.len() {
            if my_vec[index] == my_vec[next_index] && my_vec[index] != ' ' {
                counter += 1;
                my_vec[next_index] = '0';
            }
            next_index += 1;
        }
        if counter > 1 && my_vec[index] != '0' {
            result.push(my_vec[index]);
        }
        index += 1;
    }
    result
}

// This function maintains the log of "repeate_finder"
fn repeat_finder_log() {
    env_logger::init();
    if log_enabled!(Level::Info) {
        let output_value = repeat_finder("hello world");
        info!("the answer is: {}", output_value);
    }
}
