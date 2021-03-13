#[cfg(test)]

#[test]
fn first_quadrant_success(){
    let Points = quadrant::Points{x_value:1, y_value: 2 };
    let test_output = quadrant::quadrant_evaluation(&Points);
}