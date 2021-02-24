pub fn leap_counter() {
    let array=[(5,6,2020),(6,5,2024)];
    let mut counter = 0;   //counter
    let mut index=0;
    while index < array.len()  // calculating value

    {
        let temp = array[index];
        if temp.2%4==0
        {
            counter +=1;
        }
        index+=1;
    }
    print!("{}",counter);
}