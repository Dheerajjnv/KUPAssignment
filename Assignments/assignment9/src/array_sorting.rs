// _sorting function sort the given array of type generic.
//
// #Argument
// array- collection of generic type.
//
// #Return
// array- sorted array of collection generic type.

pub fn _sorting<T: PartialOrd + Copy>(array: &mut [T]) -> &[T] {
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
