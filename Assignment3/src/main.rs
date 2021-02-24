mod searching;
mod leap_year;

fn main() {
    let mut arr: [i32; 5] = [10,20,20,30,40];   //Array to input
    let search=20;  //target value
    let start=0;
    let end=arr.len()-1;
    searching::binarysearch(search, end as i32, start, arr);   //calling binary function
    searching::linearsearch(&arr, search, start);     //calling linear function
    leap_year::leap_counter();   //calling leap function
    }
