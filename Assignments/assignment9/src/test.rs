use crate::generic_operation::{array_sorting, minimum};

#[cfg(test)]
#[test]
// Minimum_calculator_test_success checks the success of minimum calculator.
fn minimum_calculator_test_success() {
    use crate::minimum;
    let output = minimum::minimum_calculator(45, 26);
    assert_eq!(output, 26);
    let output_value = minimum::_minimum_calculator(4, 26);
    assert_eq!(output_value, 4);
}

// array_sorting_success checks the success of sorting.
#[test]
fn array_sorting_success() {
    use crate::array_sorting;
    let mut input = vec![5, 9, 1, 4, 2, 6];
    let output = array_sorting::sorting(&mut input);
    let result = vec![1, 2, 4, 5, 6, 9];
    assert_eq!(output, result);
    let mut input = vec![5.6, 9.0, 1.2, 4.3, 2.3, 6.7];
    let output = array_sorting::sorting(&mut input);
    let result = vec![1.2, 2.3, 4.3, 5.6, 6.7, 9.0];
    assert_eq!(output, result);
}

// geometric_series_success check the success of geometric_series_generator.
#[test]
fn geometric_series_success() {
    use crate::geometric_series;
    use crate::geometric_series::Iterator;
    let mut series = geometric_series::GeometricSeries {
        first_number: 2,
        current_number: 1,
        ratio: 2,
    };
    let output = series.geometric_series_generator();
    assert_eq!(
        output,
        [6, 10, 18, 34, 66, 130, 258, 514, 1026, 2050, 4098, 8194]
    );
    let mut unit_series = geometric_series::GeometricSeries {
        first_number: 1,
        current_number: 1,
        ratio: 2,
    };
    let output = unit_series.geometric_series_generator();
    assert_eq!(
        output,
        [3, 5, 9, 17, 33, 65, 129, 257, 513, 1025, 2049, 4097]
    );
}
