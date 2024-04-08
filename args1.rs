// Arguments, just like Rust
use std::env;

fn main() {
    let first = env::args().nth(1).expect("Please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");

    println!("The argument: {}", n);
}