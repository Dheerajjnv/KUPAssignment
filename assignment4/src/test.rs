#[cfg(test)]
use crate::{hello_count, palindrom};

#[test]
// Testing for duplicate character.
fn duplicate_testing_success() {
    let word_hello = String::from("hello world");
    output = hello_count::repeat_finder(&word_hello);
    assert_eq!(output, "lo")
}
#[test]
fn duplicate_testing__success() {
    let word_hello = String::from("dheeraj");
    output = hello_count::repeat_finder(&word_hello);
    assert_eq!(output, "e")
}

#[test]
// Testing for checking palindrome.

fn pelindrom_testing_success() {
    let word = String::from("aabbbbaa");
    output = palindrom::check_palindrome(&word);
    assert_eq!(output, true)
}
#[test]
fn pelindrom_testing__success() {
    let word = String::from("aabbbbaaaabbbbaa");
    output = palindrom::check_palindrome(&word);
    assert_eq!(output, true)
}
#[test]
//Testing to checking rotation
fn rotation_success(){
    let word = String::from("abcd");
    let rotate = String::from("dcba");
    output = is_rotation(&word, &rotate);
    assert_eq!(output,"rotation is true")
}
#[test]
fn rotation_failure(){
    let word = String::from("abcd");
    let rotate = String::from("dbaa");
    output = is_rotation(&word, &rotate);
    assert_eq!(output,"rotation is true")
}