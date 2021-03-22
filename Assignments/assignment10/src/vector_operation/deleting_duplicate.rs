/// _delete_duplicate remove all the duplicate element present in the list.
///
///  #Argumente
///  list- Referenced mutable vector of i32.
///
///  #Return
///  store-A vector of i32 after removing all the duplicate value.

pub fn delete_duplicate(list: &mut Vec<i32>) -> Vec<i32> {
    let length = list.len();
    let mut store: Vec<i32> = Vec::new();
    for index in 0..list.len() - 1 {
        if list[index] != list[index + 1] {
            store.push(list[index]);
        }
    }
    store.push(list[length - 1]);
    store
}
/// This function checks weather a given vector have duplicate or not.
///
/// #Argument
///  list- Referenced mutable vector of i32.
///
/// #Return
/// Return a vector after removing duplicate present or error message.

pub fn duplicate_correctness(list: &mut Vec<i32>) -> Result<Vec<i32>, String> {
    let result = delete_duplicate(list).len() < list.len();
    match result {
        true => Ok(delete_duplicate(list)),
        false => Err("No duplicate element".to_string()),
    }
}
