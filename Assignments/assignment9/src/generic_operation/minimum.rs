/// This function gives the minimum between the two number.
///
/// #Argument
///
/// first_no. - Type generic.
/// second_no. -Type generic.
///
/// #Return
///
/// Returns the minimum between two numbers if number are not equal else 0.
pub fn minimum_calculator<T: Ord>(first_no: T, second_no: T) -> T {
    let result: bool = first_no > second_no;
    match result {
        true => second_no,
        false => first_no,
    }
}
