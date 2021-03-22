#[test]
/// substring_generation_success Checks the success of substring generation
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
/// pattern_value_success checks the patter_value success.
fn pattern_value_success() {
    use crate::pattern_searching;
    let output = pattern_searching::pattern_matching("Pankaj Chaudhary", "Cha");
    assert_eq!(output.unwrap(), 7);
}
#[test]
fn pattern_value_failur() {
    use crate::pattern_searching;
    let output = pattern_searching::pattern_matching("Pankaj Chaudhary", "z");
    assert_eq!(
        output,
        Err("pattern is not present in given string".to_string())
    );
}
#[test]
///string_operation_success checks get_conditional_string success
fn string_operation_success() {
    use crate::string_operation;
    let output = string_operation::get_conditional_string("jjdhid", "ikjhjk", "rtysgi");
    assert_eq!(output.unwrap(), "itdsgk")
}
#[test]
fn string_operation_secondary_success() {
    use crate::string_operation;
    let output = string_operation::get_conditional_string("abcdef", "aioeu", "ramram");
    assert_eq!(output.unwrap(), "aicra")
}
