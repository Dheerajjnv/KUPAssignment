#[cfg(test)]
#[test]
fn first_repeated_searching_success() {
    env_logger::builder().is_test(true).try_init();
    use crate::recursive_operation::enum_list::List::{Cons, Nil};
    use crate::recursive_operation::first_repeated::first_repeated_searching;
    let input = Cons(
        1,
        Box::new(Cons(
            21,
            Box::new(Cons(
                21,
                Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        )),
    );
    assert_eq!(first_repeated_searching(0, input), 21);
    log::info!("This is first repeating element result")

}
#[test]
fn second_repeated_searching_success() {
    env_logger::builder().is_test(true).try_init();
    use crate::recursive_operation::enum_list::List::{Cons, Nil};
    use crate::recursive_operation::second_repeated::second_repeated_searching;
    let input = Cons(
        2,
        Box::new(Cons(
            3,
            Box::new(Cons(
                5,
                Box::new(Cons(
                    5,
                    Box::new(Cons(
                        2,
                        Box::new(Cons(7, Box::new(Cons(7, Box::new(Cons(8, Box::new(Nil))))))),
                    )),
                )),
            )),
        )),
    );
    assert_eq!(second_repeated_searching(0, 0, input), 7);
    log::info!("This is second repeating element result")
}
#[test]
fn third_odd_searching_success() {
    env_logger::builder().is_test(true).try_init();
    use crate::recursive_operation::enum_list::List::{Cons, Nil};
    use crate::recursive_operation::third_odd::third_odd_searching;
    let input = Cons(
        1,
        Box::new(Cons(
            21,
            Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
        )),
    );
    assert_eq!(third_odd_searching(0, input), 3);
    log::info!("This is third odd element result")

}
#[test]
fn nth_number_finder_success() {
    env_logger::builder().is_test(true).try_init();
    use crate::recursive_operation::enum_list::List::{Cons, Nil};
    use crate::recursive_operation::nth_element::nth_element_finder;
    let input = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
        )),
    );
    assert_eq!(nth_element_finder(3, 0, input), 4);
    log::info!("This is nth element element result")

}
