/// This function is used to match a given pattern with string
///
/// #Arguments
///
/// string: Type str  on which pattern is going to match.
/// pattern:Type str which is to be matched on a input_string.
///
/// #Return
///
/// Return a result index value if found else error message.
pub fn pattern_matching(string: &str, pattern: &str) -> Result<usize, String> {
    let collect_string: Vec<char> = string.chars().collect();
    let collect_pattern: Vec<char> = pattern.chars().collect();
    let mut count = 0;
    let mut take_index_value;

    for index in 0..(collect_string.len() - collect_pattern.len() + 1) {
        take_index_value = index;

        for travel in &collect_pattern {
            if travel == &collect_string[take_index_value] {
                count += 1;
            }
            if count == collect_pattern.len() {
                return  Ok(index);
            }
            take_index_value += 1;
        }
        count = 0;
    }
    log::warn!("Pattern not found");
    Err("pattern is not present in given string".to_string())
}