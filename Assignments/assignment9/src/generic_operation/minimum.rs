/// This function gives the minimum between the two number.
///
/// #Argument
///
/// first_no. - Type generic.
/// second_no. -Type generic.
///
/// #Return
///
/// Returns the minimum between two numbers if number are not equal else error message.
pub fn minimum_result<T: std::cmp::PartialOrd>(first_no: T, second_no: T) -> Result<T, String> {
    if first_no < second_no {
        Ok(first_no)
    } else if first_no > second_no {
        Ok(second_no)
    } else {
        log::error!("Error for comparing 2 value");
        Err("Hey both numbers are equal".to_string())
    }
}
