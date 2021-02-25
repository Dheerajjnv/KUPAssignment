mod searching;
mod leap_year;
mod sorting;

fn main(){
    let mut arr: [i32; 5] = [10, 20, 20, 30, 40];   //Array to input
    let search = 20;  //target value
    let start = 0;
    let end = arr.len() - 1;
    searching::binarysearch(search, end as i32, start, arr);   //calling binary function
    searching::linearsearch(&arr, search, start);     //calling linear function
    leap_year::leap_counter();   //calling leap function
    println!("\n");
    let mut nums = vec![1.2, 3.3, 7.9, 0.1, 3.6, 20.6, 1.5]; // array to be sorted
    let l = nums.len();
    sorting::merge_sort(&mut nums, 0, l - 1); // calling merge sort
    for f in nums {
        print!("{} ", f);
    }
}
