
#[test]
// Function Checks the success of substring generation
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
// Function checks the patter_value success.
fn pattern_value_success() {
    use crate::pattern_searching;
    let output = pattern_searching::pattern_value();
    assert_eq!(output, 7);
}
