#[cfg(test)]
#[test]
//Testing first quadrant.
fn first_quadrant_success() {
    use crate::quadrant;
    let points = quadrant::_Points { x: 1, y: 2 };
    let test_output = quadrant::quadrant_evaluation(&points);
    assert_eq!(
        test_output,
        "Position::First(Coordinate::Abscissa 1, Coordinate::Ordinate 2 ".to_string()
    );
}
#[test]
//Testing second quadrant.
fn second_quadrant_success() {
    use crate::quadrant;
    let points = quadrant::_Points { x: -1, y: 2 };
    let test_output = quadrant::quadrant_evaluation(&points);
    assert_eq!(
        test_output,
        "Position::Second(Coordinate::Abscissa -1, Coordinate::Ordinate 2 ".to_string()
    );
}
#[test]
//Testing third quadrant.
fn third_quadrant_success() {
    use crate::quadrant;
    let points = quadrant::_Points { x: 1, y: -2 };
    let test_output = quadrant::quadrant_evaluation(&points);
    assert_eq!(
        test_output,
        "Position::Third(Coordinate::Abscissa 1, Coordinate::Ordinate -2 ".to_string()
    );
}
#[test]
//Testing fourth quadrant.
fn fourth_quadrant_success() {
    use crate::quadrant;
    let points = quadrant::_Points { x: -1, y: -2 };
    let test_output = quadrant::quadrant_evaluation(&points);
    assert_eq!(
        test_output,
        "Position::Fourth(Coordinate::Abscissa -1, Coordinate::Ordinate -2 ".to_string()
    );
}
