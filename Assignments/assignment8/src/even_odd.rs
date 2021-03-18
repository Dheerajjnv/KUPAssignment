/// This function checks weather a number is even or not.
///
/// #Argument
/// number - Integer of type i32.
///
/// #Return
/// This returns the String type result 'number is even' for even numbers and 'Oh, number is not even' for odd numbers.
pub fn even_number_evaluator(number: i32) -> String {
    let result = even_number_examiner(number);
    match result {
        Ok(_number) => ("number is even").to_string(),
        Err(_msg) => ("Oh, number is not even").to_string(),
    }
}

/// This function examine weather a number is even or not.
///
/// #Argument
/// number - Integer of type i32.
///
///  #Return
///
/// This returns the String type result 'This is a even no.' for even numbers and 'oh, number is not even' for odd numbers.

pub fn even_number_examiner(number: i32) -> Result<String, String> {
    if number % 2 == 0 {
        Ok("This is a even no.".to_string())
    } else {
        Err("oh number is not even".to_string())
    }
}
