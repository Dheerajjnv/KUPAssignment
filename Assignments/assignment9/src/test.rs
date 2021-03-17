use crate::generic_operation::{array_sorting, minimum};
use crate::trait_operation::geometric_series::{Iterator,GeometricSeries};


#[cfg(test)]
#[test]
fn minimum_calculator_equality_success() {
    let output = minimum::minimum_calculator(45, 45);
    assert_eq!(output, 45);
}
#[test]
fn minimum_calculator_inequality_success() {
    let output = minimum::minimum_calculator(45, 26);
    assert_eq!(output, 26);
}
#[test]
fn array_sorting_integral_success() {
    let mut input = vec![5, 9, 1, 4, 2, 6];
    let output = array_sorting::sorting(&mut input);
    let result = vec![1, 2, 4, 5, 6, 9];
    assert_eq!(output, result);
}
#[test]
fn array_sorting_fraction_success() {
    let mut input = vec![5.6, 9.0, 1.2, 4.3, 2.3, 6.7];
    let output = array_sorting::sorting(&mut input);
    let result = vec![1.2, 2.3, 4.3, 5.6, 6.7, 9.0];
    assert_eq!(output, result);
}
#[test]
fn geometric_series_success() {
    let mut series = GeometricSeries{
        first_number: 2,
        current_number: 1,
        ratio: 2,
    };
    let output = series.geometric_series_generator();
    assert_eq!(
        output,
        [6, 10, 18, 34, 66, 130, 258, 514, 1026, 2050, 4098, 8194]
    );
}
#[test]
fn geometric_series_unit_success() {
    let mut series = GeometricSeries{
        first_number: 1,
        current_number: 0,
        ratio: 1,
    };
    let output = series.geometric_series_generator();
    assert_eq!(
        output,
        [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
    );
}