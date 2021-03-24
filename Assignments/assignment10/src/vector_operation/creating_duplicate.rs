/// duplicate_lelement create the duplicate of each element present in the list.
///
///  #Argument
/// list-A reference type mutable vector of i32.
///
///  #Return
///  list -A list of type vector containing original and duplicate element.
pub fn duplicate_element(list: &mut Vec<i32>) -> Vec<i32> {
    let length = list.len();
    for index in 0..length {
        let value = list[index];
        list.push(value);
        list.push(value);
    }
    list[length..].to_vec()
}
