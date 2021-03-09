pub fn linearsearch(array:&[i32] ,x: i32,target: i32)
{
    if array.len() == target as usize
    {
        println!("Number is not present");
        return;
    }
    if array[target as usize] == x
    {
        println!("{}" , target);
        return ;
    }
    linearsearch(array,  x , target+1);
}


pub fn binarysearch(search:i32,end:i32,start:i32, array: [i32;5]) {
    if start > end

    {
        return;
    }
    let mid=(start+end)/2;

    if mid==search
    {

        print!("{}",mid);
    }
    else if search>array[mid as usize ]
    {
        binarysearch(search,end,mid+1,array);

        print!("{}",mid);
    }
    else {
        binarysearch(search,mid-1,start,array);

        print!("{ }",mid);
    }


}