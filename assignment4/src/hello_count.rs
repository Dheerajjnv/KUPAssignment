//Function Checking the repeating character in a word
//
//#Argument
//word
//
//Return
//repeating character
pub mod hello_count {
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
        return result;
    }
}
