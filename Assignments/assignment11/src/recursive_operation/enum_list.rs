///List - An enum type data
/// #Varient
///
/// Cons - integer of type i32 and Box<List>
/// Nil - Value to be none like.
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
