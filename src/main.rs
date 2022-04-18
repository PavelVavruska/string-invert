use std::io::{stdin, Read};

pub mod inv;

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("Error.");
    let inverted_string = inv::inverse_words(&buf);
    println!("{}", inverted_string);
}
