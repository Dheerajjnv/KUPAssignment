use crate::recursive_operation::enum_list::List;
use crate::recursive_operation::enum_list::List::{Cons, Nil};
pub fn first_repeated_searching(first_number: i32, list: List) -> i32 {
    match list {
        Cons(next_number, list) => match first_number == next_number {
            true => next_number,
            false => first_repeated_searching(next_number, *list),
        },
        Nil => -1,
    }
}
