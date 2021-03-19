use async_std::task;
use std::time::Duration;

/// This asynchronous function calculate the table of 2.
pub async fn table_2() {
    for i in 1..11{
        let result = 2 * i;
        task::sleep(Duration::from_millis(50));
        println!("This is the value table {}", result);
    }
}

/// This asynchronous function calculate the table of 2.
pub async fn table_3() {
    for i in 1..11 {
        let result = 3 * i;
        task::sleep(Duration::from_millis(50));
        println!("This is the value table {}", result);
    }
        log::info!("This is the value table 3")
    }

/// This asynchronous function run both above function parallel till the end of thread.
pub async fn main_executor() {
    use futures::future::join;
    join(table_2(), table_3()).await;
}
