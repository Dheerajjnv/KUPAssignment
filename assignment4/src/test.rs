#[cfg(test)]
use crate::{palindrom, hello_count};

#[test]
// Testing for duplicate character.
fn duplicate_testing() {
    let word_hello = String::from("hello world");
    output = hello_count::repeat_finder(&word_hello);
    assert_eq!(output, "lo")
}
#[test]
fn duplicate_testing_success() {
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
fn pelindrom__testing_success() {
    let word = String::from("aabbbbaaaabbbbaa");
    output = palindrom::check_palindrome(&word);
    assert_eq!(output, true)
}
