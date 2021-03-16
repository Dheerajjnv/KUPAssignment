/// _delete_duplicate remove all the duplicate element present in the list.
///
///  #Argumente
///  list- Referenced mutable vector of i32.
///
///  #Return
///  store-A vector of i32 after removing all the duplicate value.

pub fn _delete_duplicate(list: &mut Vec<i32>) -> Vec<i32> {
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
