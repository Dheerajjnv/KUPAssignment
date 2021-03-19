/// first_even return the first even number present in the given list of number
///
///  #Argument
///  list-A vector of i32.
///
///  #Return
///  result -The first even number of list type i32.
pub fn _first_even(list: &mut Vec<i32>) -> i32 {
    let mut result = 0;
    let mut index = 0;
    while index < list.len() - 1 {
        if list[index] % 2 == 0 {
            result = list[index];
            break;
        } else {
            log::warn!("No correct input");
            result = 0
        }
        index += 1;
    }
    result
}
