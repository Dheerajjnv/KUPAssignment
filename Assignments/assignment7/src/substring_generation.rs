/// Function sub_string_generation generate all possible substring of a given string
///
/// #Argument
/// word-string type.
///
/// #Return
/// store_substring- Vector type storing all the string.
pub fn sub_string_generator(word: String) -> Vec<String> {
    let mut store_substring = Vec::new();
    let mut collect_str: &str;
    for i in 0..word.len() {
        for j in 0..word.len() {
            collect_str = &word[i..(j + 1)];
            store_substring.push(String::from(collect_str));
        }
    }
    store_substring
}
pub fn substring_check(word: String) -> Result<Vec<String>, String> {
    let result = word.is_empty();
    match result {
        false => Ok(sub_string_generator(word)),
        true => Err("Empty string".to_string()),
    }
}
