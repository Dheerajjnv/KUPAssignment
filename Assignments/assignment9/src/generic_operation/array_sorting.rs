/// This function sort the given array of generic type.
///
/// #Argument
///
/// array - A vector of generic type .
///
/// #Return
///
/// Return a generic type sorted array.

pub fn sorting<T: PartialOrd + Copy>(array: &mut [T]) -> &[T] {
    let mut temp;
    let length = array.len();
    for index in 0..length {
        for travel_index in index + 1..length {
            if array[index] > array[travel_index] {
                temp = array[index];
                array[index] = array[travel_index];
                array[travel_index] = temp;
            }
        }
    }
    array
}
