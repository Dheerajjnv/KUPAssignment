// even response return results weather a no.is even or not.
//
// #Argument
// number-type i32
//
// #Return
// String.
pub fn even_response(number: i32) -> String {
    let result = even_calculator(number);
    match result {
        Ok(_number) => ("no is even").to_string(),
        Err(_msg) => ("Oh no is not even").to_string(),
    }
}

// even_calculator check weather a no.is even or not.
//
// #Argument
// number-type i32
//
// #Return
// <String, String>-Type result

pub fn even_calculator(number: i32) -> Result<String, String> {
    if number % 2 == 0 {
        Ok("This is a even no.".to_string())
    } else {
        Err("oh number is not even".to_string())
    }
}
