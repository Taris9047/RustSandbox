// matching 
use std::env;

fn main() {
    let first = env::args().nth(1).expect("Need an Integer!");
    let n: i32 = first.parse().expect("Not an Integer!");

    let num = match n {
        0..=3 => "small",
        4..=6 => "medium",
        _ => "large",
    };

    println!("Arg is...{}...", num);
}