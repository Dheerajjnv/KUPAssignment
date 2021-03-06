//Function Checking the Palindrome condition in a word
//
//#Argument
//word
//
//Return
//true,false

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
