/// _palindrome_check check the palindrome condition in the given vector.
///
///  #Argument
///  list-Reference mutable type vector of i32.
///
/// #Return
/// result-Type boolean{true, false}
pub fn _palindrome_check(list: &mut Vec<i32>) -> bool {
    let mid = list.len() / 2;
    let mut front = 0;
    let mut back = list.len() - 1;
    let mut result: bool = false;
    while front < mid && back > mid {
        if list[front] == list[back] {
            back -= 1;
            front += 1;
            result = true;
        } else {
            log::info!("Not a palindrome list");
            result = false;
        }
    }
    result
}
