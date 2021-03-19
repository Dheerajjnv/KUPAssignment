use assignment12::printing_table::main_executor;
use assignment12::async_data::async_data;

fn main() {
    use futures::executor::block_on;
    block_on(main_executor());
    async_data()
}