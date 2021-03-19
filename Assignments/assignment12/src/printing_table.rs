use async_std::task;
use std::time::Duration;

/// This asynchronous function print the table of 2 and 3.
pub async fn print_table() {
    use futures::future::join;
    let table_first = async {
        for i in 1..11 {
            let result = 2 * i;
            println!("{}", result);
        }
        task::sleep(Duration::from_secs(1)).await;
    };
    let table_second = async {
        for i in 1..11 {
            let result = 3 * i;
            println!("{}", result);
        }
        task::sleep(Duration::from_secs(1)).await;
    };
    join(table_first, table_second).await;
    log::info!("The table printed asynchronously")
}
