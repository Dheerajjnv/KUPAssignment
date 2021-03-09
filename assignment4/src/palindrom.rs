use log::{debug, error, info, log_enabled, Level};

// Function check the palindrome condition in a word
//
// #Argument
// Gets a "string" as argument
//
//Return
//This will return boolean{true,false} after checking the palindrome condition

pub fn check_palindrome(word: &str) -> bool {
    if word.len() == 0 {
        return true;
    }
    let mut end_index = word.len() - 1;
    let mut start_index = 0;

    let my_vec = word.as_bytes().to_owned();

    while start_index < end_index {
        if my_vec[start_index] != my_vec[end_index] {
            return false;
        }

        start_index += 1;
        end_index -= 1;
    }
    return true;
}

// This function create the log of "check_palindrome"
fn check_palindrome_log() {
    env_logger::init();
    if log_enabled!(Level::Info) {
        let output_value = check_palindrome("abccba");
        info!("the answer is: {}", output_value);
    }
}