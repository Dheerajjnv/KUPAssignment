
pub fn desired_output_fn(first_slice: &str, second_slice: &str, third_slice: &str) -> String {
    let mut len_iter = 0;
    let mut index_value: usize = 0;
    let mut output: String = String::new();
    let mut _first_compare = 0;
    if len_iter %2 ==0 {_first_compare = 0} else { _first_compare =1};
    while index_value < first_slice.len() && index_value < second_slice.len() && index_value < second_slice.len() {
        match _first_compare {
            0 => {
                let comp_res_str1_str2 =
                    if first_slice.chars().nth(index_value) < second_slice.chars().nth(index_value) {
                        first_slice.chars().nth(index_value)
                    } else {
                        second_slice.chars().nth(index_value)
                    };
                let res_char = if comp_res_str1_str2 < third_slice.chars().nth(index_value) {
                    comp_res_str1_str2
                } else {
                    third_slice.chars().nth(index_value)
                };
                if let Some(_t) = res_char {
                    output.push(res_char.unwrap())
                }
            }

            1 => {
                let comp_res_str1_str2 =
                    if first_slice.chars().nth(index_value) < second_slice.chars().nth(index_value) {
                        second_slice.chars().nth(index_value)
                    } else {
                        first_slice.chars().nth(index_value)
                    };
                let result = if comp_res_str1_str2 > third_slice.chars().nth(index_value) {
                    comp_res_str1_str2
                } else {
                    third_slice.chars().nth(index_value)
                };
                if let Some(_t) = result {
                    output.push(result.unwrap())
                }
            }
            _ => {}
        }

        index_value += 1;
        len_iter += 1;
    }
    output
}