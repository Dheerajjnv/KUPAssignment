/// get_conditional_string return the output on given condition.
///
/// #Argument.
/// first_slice-Reference variable of type str.
/// second_slice-Reference variable of type str.
/// third_slice-Reference variable of type str.
///
/// #Return
/// Return the output of option type string having desired condition.
pub fn get_conditional_string(
    first_slice: &str,
    second_slice: &str,
    third_slice: &str,
) -> Option<String> {
    let mut position = 0;
    let mut travel_index: usize = 0;
    let mut array: Vec<char> = Vec::new();

    while travel_index < second_slice.len() {
        if position % 2 == 0 {
            let compare_result =
                if first_slice.chars().nth(travel_index) < second_slice.chars().nth(travel_index) {
                    first_slice.chars().nth(travel_index)
                } else {
                    second_slice.chars().nth(travel_index)
                };
            let result_char = if compare_result < third_slice.chars().nth(travel_index) {
                compare_result
            } else {
                third_slice.chars().nth(travel_index)
            };
            if let Some(_t) = result_char {
                array.push(result_char.unwrap());
            }
        } else {
            let compare_result =
                if first_slice.chars().nth(travel_index) > second_slice.chars().nth(travel_index) {
                    first_slice.chars().nth(travel_index)
                } else {
                    second_slice.chars().nth(travel_index)
                };
            let result_char = if compare_result > third_slice.chars().nth(travel_index) {
                compare_result
            } else {
                third_slice.chars().nth(travel_index)
            };
            if let Some(_t) = result_char {
                array.push(result_char.unwrap());
            }
        }
        travel_index += 1;
        position += 1
    }
    let result: String = array.iter().collect();
    Some(result)
}
