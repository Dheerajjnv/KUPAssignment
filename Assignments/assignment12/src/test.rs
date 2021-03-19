#[test]
fn async_data_generator_testing() {
    use crate::async_data::async_data;
    assert_eq!(async_data(), ())
}
#[test]
fn print_table_success() {
    use crate::printing_table::print_table;
    use futures::executor::block_on;
    assert_eq!(block_on(print_table()), ())
}
