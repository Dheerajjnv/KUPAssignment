use crate::recursive_operation::enum_list::List;
use crate::recursive_operation::enum_list::List::{Cons, Nil};
/// This function gives the first repeating element.
///
/// #Argument
///
/// first_number - first number of type integer i32.
/// list - A List of type enum containing Box<i32> and Nil.
///
/// #Return
///
/// Return the first repeating element from a give collection of element.
pub fn first_repeated_searching(first_number: i32, list: List) -> i32 {
    match list {
        Cons(next_number, list) => match first_number == next_number {
            true => next_number,
            false => first_repeated_searching(next_number, *list),
        },
        Nil => 0,
    }
}
