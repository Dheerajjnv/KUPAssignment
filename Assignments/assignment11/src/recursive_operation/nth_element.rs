use crate::recursive_operation::enum_list::List;
use crate::recursive_operation::enum_list::List::{Cons, Nil};
/// This function gives the nth index element.
///
/// #Argument
///
/// index - index value of type integer i32.
/// result - result of type integer i32.
/// list - A List of type enum containing Box<i32> and Nil.
///
/// #Return
///
/// Return the index nth element from a give collection of element.
pub fn nth_element_finder(index: i32, result: i32, list: List) -> i32 {
    match list {
        Cons(number, list) => {
            if index == result {
                number
            } else {
                nth_element_finder(index, result + 1, *list)
            }
        }
        Nil => 0,
    }
}
