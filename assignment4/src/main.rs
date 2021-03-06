mod hello_count;
mod pelindrom;
mod rotation;
use cargo_tarpaulin::config::Config;
use log::{warn, LevelFilter};
use simple_logger::{Config, TermLogger, TerminalMode};

//Driver_code
fn main() {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    let word_hello = String::from("hello world");
    warn!("{}", hello_count::repeat_finder(&word_hello));
    let word = String::from("aabbbbaa");
    warn!("{}", pelindrom::check_palindrome(&word));
    let actual_word = String::from("abcde");
    let rotated_word = String::from("bcdea");
    warn!("{}", rotation::is_rotation(actual_word, rotated_word));
}
