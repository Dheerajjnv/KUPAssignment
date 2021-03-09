use log::{debug, error, info, log_enabled, Level};

// This function checks weather a word is the rotation of another word or not.
//
// #Argument
// string1-A reference parameter of type str for actual word.
// string2-A reference parameter of type str for ratated word.
//
// Return
// Return boolean{true, false} after checking condition that whether a "string2" is the rotation of "string1" or not.

pub fn is_rotation(string1: &str, string2: &str) -> String {
    let mut collect_reverse_char = String::new();
    let mut output = String::new();
    for count in string1.chars().rev() {
        collect_reverse_char.push(count);
    }
    if collect_reverse_char == string2 {
        output.push_str("rotation is true");
    } else {
        output.push_str("rotation is false");
    }
    output
}

// Function maintaining the log for "is_rotation"
fn repeat_finder_log() {
    env_logger::init();
    if log_enabled!(Level::Info) {
        let output_value = is_rotation("aabbbbaa", "bbbbaaaa");
        info!("the answer is: {}", output_value);
    }
}
