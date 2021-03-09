use log::{debug, error, info, log_enabled, Level};

// This function checks weather a word is the rotation of another word or not.
//
// #Argument
// Gets two word  actual_word ,rotated_word as argument.
//
// Return
// This will return boolean{true, false} after checking condition that whether a word is the rotation of other or not.

pub fn is_rotation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }
    string1.repeat(2).contains(&string2)
}
fn repeat_finder_log() {
    env_logger::init();
    if log_enabled!(Level::Info) {
        let output_value = is_rotation("aabbbbaa", "bbbbaaaa");
        info!("the answer is: {}", output_value);
    }
}
