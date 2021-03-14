// minimum_calculation return the minimum between the two no.
//
// #Argument
// first_no. - type generic
// second_no. -type generic
// #Return
// Minimum of type T
pub fn _minimum_calculator<T: Ord>(first_no: T, second_no: T) -> T {
    let result: bool = first_no > second_no;
    match result {
        true => second_no,
        false => first_no,
    }
}
