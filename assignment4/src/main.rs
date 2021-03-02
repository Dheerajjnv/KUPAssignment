mod hello_count;
mod pelindrom;
mod rotation;

fn main() {
    println!("Strings : {}", rotation::is_rotation("abcd".parse().unwrap(), "dcba".parse().unwrap()));
    println!("String is : {}", pelindrom::check_palindrome("acba"));
    println!("Duplicate Characters : {}", hello_count::find_rep_char("Hello World"));
}