#[cfg(test)]
#[test]
fn minimum_calculator_equality_success() {
    use crate::generic_operation;
    let output = generic_operation::minimum::minimum_result(45, 45);
    assert_eq!(output, Err("Hey both numbers are equal".to_string()))
}

#[test]
fn minimum_calculator_inequality_success() {
    use crate::generic_operation;
    let output = generic_operation::minimum::minimum_result(45, 26).unwrap();
    assert_eq!(output, 26);
}

#[test]
fn array_sorting_integral_success() {
    use crate::generic_operation;
    let mut input = vec![5, 9, 1, 4, 2, 6];
    let output = generic_operation::array_sorting::sorting(&mut input);
    let result = vec![1, 2, 4, 5, 6, 9];
    assert_eq!(output, result);
}

#[test]
fn array_sorting_fraction_success() {
    use crate::generic_operation;
    let mut input = vec![5.6, 9.0, 1.2, 4.3, 2.3, 6.7];
    let output = generic_operation::array_sorting::sorting(&mut input);
    let result = vec![1.2, 2.3, 4.3, 5.6, 6.7, 9.0];
    assert_eq!(output, result);
}

#[test]
fn geometric_series_success() {
    use crate::trait_operation;
    let mut series = trait_operation::geometric_series::GeometricSeries {
        first_number: 2,
        current_number: 1,
        ratio: 2,
    };
    use crate::trait_operation::geometric_series::Iterator;
    let output = series.geometric_series_generator();
    assert_eq!(
        output,
        [6, 10, 18, 34, 66, 130, 258, 514, 1026, 2050, 4098, 8194]
    );
}

#[test]
fn geometric_series_unit_success() {
    use crate::trait_operation;
    let mut series = trait_operation::geometric_series::GeometricSeries {
        first_number: 1,
        current_number: 0,
        ratio: 1,
    };
    use crate::trait_operation::geometric_series::Iterator;
    let output = series.geometric_series_generator();
    assert_eq!(output, [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
}
