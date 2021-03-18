use crate::recursive_operation::enum_list::List;
use crate::recursive_operation::enum_list::List::{Cons, Nil};
/// This function gives the third odd element.
///
/// #Argument
///
/// index_travel - index value of type integer i32.
/// list - A List of type enum containing Box<i32> and Nil.
///
/// #Return
///
/// Return the third odd element from a given collection of element.
pub fn third_odd_searching(index_travel: i32, list: List) -> i32 {
    match list {
        Cons(number, list) => {
            let value = number % 2 != 0;
            match value {
                true => {
                    let flag = index_travel == 1 || index_travel == 0;
                    match flag {
                        true => third_odd_searching(index_travel + 1, *list),
                        false => number,
                    }
                }
                false => third_odd_searching(index_travel, *list),
            }
        }
        Nil => 0,
    }
}
