#[test]
// substring_generation_success Checks the success of substring generation
fn substring_generation_success() {
    use crate::substring_generation;
    let word = "pa".to_string();
    let output = substring_generation::sub_string_generator(word);
    assert_eq!(output, ["p", "pa", "", "a"]);
}
#[test]
fn substring_generation_success_secondchars() {
    use crate::substring_generation;
    let word = "ab".to_string();
    let output = substring_generation::sub_string_generator(word);
    assert_eq!(output, ["a", "ab", "", "b"]);
}

#[test]
// pattern_value_success checks the patter_value success.
fn pattern_value_success() {
    use crate::pattern_searching;
    let output = pattern_searching::pattern_value();
    assert_eq!(output, 7);
}

#[test]
//string_operation_success checks get_conditional_string success
fn string_operation_success() {
    use crate::string_operation;
    let output = string_operation::_get_conditional_string("jjdhid", "ikjhjk", "rtysgi");
    assert_eq!(output, "ijdhgd")
}
#[test]
fn string_operation_secondry_success() {
    use crate::string_operation;
    let output = string_operation::_get_conditional_string("abcdef", "aioeu", "ramram");
    assert_eq!(output, "aacda")
}
