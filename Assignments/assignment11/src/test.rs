#[cfg(test)]
#[test]
fn first_repeated_searching_success() {
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
}
#[test]
fn second_repeated_searching_success() {
    use crate::recursive_operation::enum_list::List::{Cons, Nil};
    use crate::recursive_operation::second_repeated::second_repeated_searching;
    let input = Cons(
        2,
        Box::new(Cons(
            3,
            Box::new(Cons(
                5,
                Box::new(Cons(5, Box::new(Cons(2, Box::new(Cons(7, Box::new(Cons(7, Box::new(Cons(8, Box::new(Nil))))))))))),
            )),
        )),
    );
    assert_eq!(second_repeated_searching(0, 0,input), 7);
}
