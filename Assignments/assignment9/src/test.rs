#[cfg(test)]
#[test]
// Minimum_calculator_test_success checks the success of _minimum calculator.
fn minimum_calculator_test_success() {
    use crate::minimum;
    let output = minimum::_minimum_calculator(45, 26);
    assert_eq!(output, 26);
    let output_value = minimum::_minimum_calculator(4, 26);
    assert_eq!(output_value, 4);
}

// array_sorting_success checks the success of sorting.
#[test]
fn array_sorting_success() {
    use crate::array_sorting;
    let mut input = vec![5, 9, 1, 4, 2, 6];
    let output = array_sorting::_sorting(&mut input);
    let result = vec![1, 2, 4, 5, 6, 9];
    assert_eq!(output, result);
    let mut input = vec![5.6, 9.0, 1.2, 4.3, 2.3, 6.7];
    let output = array_sorting::_sorting(&mut input);
    let result = vec![1.2, 2.3, 4.3, 5.6, 6.7, 9.0];
    assert_eq!(output, result);
}
