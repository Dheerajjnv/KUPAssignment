use std::thread;
use std::time::Duration;
/// This function print the data in asynchronous manner.
pub fn async_data(){
    let first_value = thread::spawn(|| {
        for i in 1..10 {
            println!("{}" , i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{}" , i);
        thread::sleep(Duration::from_millis(1));
    }
    first_value.join().unwrap()
}