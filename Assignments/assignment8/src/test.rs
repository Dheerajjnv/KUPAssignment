#[cfg(test)]
#[test]
// even_calculater_success check the even calculator OK result
fn even_calculator_success() {
    use crate::even_odd;
    let output = even_odd::even_response(16);
    assert_eq!(output, "no is even".to_string())
}
#[test]
// even_calculater_success check the even calculator Err result
fn even_calculator_success_oddcase() {
    use crate::even_odd;
    let output = even_odd::even_response(17);
    assert_eq!(output, "Oh no is not even".to_string())
}
