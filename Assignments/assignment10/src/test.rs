#[cfg(test)]
#[test]
/// Testing performed to check the success of '_reverse_list'.
fn reverse_list_success() {
    use crate::reverse_list;
    let input = vec![1, 2, 3, 4, 5];
    let output = reverse_list::_reverse_list(input);
    log::info!("Testing for reverse list is done here");
    assert_eq!(output, [5, 4, 3, 2, 1])
}
#[test]
/// Testing performed to check the success of '_first_even'.
fn even_success() {
    use crate::first_even;
    let mut input = vec![9, 2, 1, 2, 3, 4, 5];
    let output = first_even::_first_even(&mut input);
    assert_eq!(output, 2)
}
#[test]
/// Testing performed to check the failure of 'first_even'.
fn even_failure() {
    use crate::first_even;
    let mut input = vec![1, 7, 5, 9, 13];
    let output = first_even::_first_even(&mut input);
    assert_eq!(output, 0);
}
#[test]
/// Testing performed to check the success of 'removing_nth_element'.
fn nth_value_removal_success() {
    use crate::removing_nth_element;
    let input = vec![1, 7, 5, 6, 8];
    let output = removing_nth_element::_removing_nth_element(input, 3);
    assert_eq!(output, [1, 7, 5, 8]);
}
#[test]
/// Testing performed to check the success of 'removing_nth_element'.
fn nth_value_removal_failure() {
    use crate::removing_nth_element;
    let input = vec![1, 7, 5, 6, 8];
    let output = removing_nth_element::_removing_nth_element(input, 8);
    assert_eq!(output, [1, 7, 5, 6, 8, -1]);
}

#[test]
/// Testing performed to check the success og 'palindrome_check'.
fn palindrome_check_success() {
    use crate::palindrome_check;
    let mut input = vec![1, 2, 3, 2, 1];
    let output = palindrome_check::_palindrome_check(&mut input);
    assert_eq!(output, true);
}
#[test]
/// Testing performed to check the failure of 'palindrome_check'.
fn palindrome_check_failure() {
    use crate::palindrome_check;
    let mut input = vec![1, 2, 3, 2, 4];
    let output = palindrome_check::_palindrome_check(&mut input);
    assert_eq!(output, false);
}
#[test]
///
fn crating_duplicate_success(){
    use crate::creating_duplicate;
    let mut input = vec![1, 2, 3];
    let output = creating_duplicate::_duplicate_element(&mut input);
    assert_eq!(output,[1,1,2,2,3,3]);
}