#[cfg(test)]
#[test]
fn even_calculator_success() {
    use crate::even_odd;
    let output = even_odd::even_number_evaluator(16);
    assert_eq!(output, "number is even".to_string())
}
#[test]
fn even_calculator_failure() {
    use crate::even_odd;
    let output = even_odd::even_number_evaluator(17);
    assert_eq!(output, "Oh no is not even".to_string())
}
