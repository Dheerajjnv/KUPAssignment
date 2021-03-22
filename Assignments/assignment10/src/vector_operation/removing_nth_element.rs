/// _removing_nth_element remove the element from the nth position if the position given is correct.
///
/// #Argument
/// list-Mut vector of type i32.
/// nth_value-position of type usize.
///
///  #Return
/// 'list' ofter removing the nth position value or 'list' ending with -1 for beyond index case.

pub fn removing_nth_element(mut list: Vec<i32>, nth_value: usize) -> Vec<i32> {
    let index = nth_value;
    if list.is_empty() {
        log::warn!("Hey input the list")
    } else if index > list.len() {
        list.push(-1_i32);
        log::error!("Beyond index")
    } else {
        list.remove(nth_value);
    }
    list
}
