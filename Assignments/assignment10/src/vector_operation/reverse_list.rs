/// reverse_list reverse the element of given list
///
/// #Argument
/// list-Type vector og i32.
///
/// #Return
/// list-Reverse the list of element type vector i32.
pub fn _reverse_list(mut list: Vec<i32>) -> Vec<i32> {
    let mut start_index = 0;
    let mut end_index = list.len() - 1;
    while start_index < end_index {
        list.swap(start_index, end_index);
        start_index += 1;
        end_index -= 1;
    }
    list
}
