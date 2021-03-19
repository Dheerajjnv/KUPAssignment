use std::thread;
use std::time::Duration;
/// This function print the data in asynchronous manner.
pub fn async_data() {
    let first_value = thread::spawn(|| {
        for i in 1..10 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 11..15 {
        println!("{}", i);
        thread::sleep(Duration::from_millis(50));
    }
    first_value.join().unwrap()
}
