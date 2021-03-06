//Function Checking the rotation of a word
//
//#Argument
//actual_word ,rotated_word
//
//Return
//true, false

pub fn is_rotation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }
    string1.repeat(2).contains(&string2)
}
