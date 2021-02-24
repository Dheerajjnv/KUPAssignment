/*To calcate the febonacci series*/
pub fn fabonaccicalculater(){
    let number = 100;
    let mut firstterm = 0;
    let mut secondterm =1;
    let mut counter = 0;
    //when number = 0
    if number==1{
        println!("fibonacci series = {}",firstterm);
    }
    //when number = 1
    else if number==2 {
        println!("fibonacci series = {}{}",firstterm,secondterm);
    }
    //when number is greater than 2
    else {
        println!("fibonaccci series");
        while counter<number {
            println!("{}",counter);
            firstterm = secondterm;
            secondterm = counter;
            counter = firstterm + secondterm;
        }
    }

}
