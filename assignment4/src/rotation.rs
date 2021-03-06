//Function Checking the rotation of a word
//
//#Argument
//actual_word ,rotated_word
//
//Return
//true, false

pub fn is_rotation(actual_word: String, rotated_word: String) -> bool {
    let len1 = actual_word.len();
    let len2 = rotated_word.len();
    let string1: Vec<char> = actual_word.chars().collect();
    let string2: Vec<char> = rotated_word.chars().collect();
    if len1 != len2 {
        return false;
    }
    let mut longest_prefix_suffix = Vec::with_capacity(len1);
    let mut prev_len = 0;
    let mut index = 1;
    longest_prefix_suffix[0] = 0;

    while index < len1 {
        if string1[index] == string2[prev_len] {
            longest_prefix_suffix[index] = prev_len + 1;
            index = index + 1;
        } else {
            if prev_len == 0 {
                longest_prefix_suffix[index] = 0;
                index = index + 1;
            } else {
                prev_len = longest_prefix_suffix[prev_len - 1];
            }
        }
    }
    index = 0;
    let mut next_index = longest_prefix_suffix[len1 - 1];
    while next_index < len2 {
        if string2[next_index] != string1[index] {
            return false;
        }
        next_index = next_index + 1;
    }
    return true;
}
