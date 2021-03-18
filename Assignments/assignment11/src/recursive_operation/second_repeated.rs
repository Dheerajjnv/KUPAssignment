use crate::recursive_operation::enum_list::List;
use crate::recursive_operation::enum_list::List::{Cons, Nil};
/// This function gives the second repeating element.
///
/// #Argument
///
/// first_number - first number of type integer i32.
/// counter - counter of type integer i32.
/// list - A List of type enum containing Box<i32> and Nil.
///
/// #Return
///
/// Return the second repeating element from a given collection of element.
pub fn second_repeated_searching(counter: i32, first_number: i32, list: List) -> i32 {
    match list {
        Cons(next_number, list) => {
            if first_number == next_number && counter == 0 {
                second_repeated_searching(counter + 1, next_number, *list)
            } else if first_number == next_number && counter == 1 {
                next_number
            } else {
                second_repeated_searching(counter, next_number, *list)
            }
        }
        Nil => 0,
    }
}
