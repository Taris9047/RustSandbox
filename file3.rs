// Simplified error handling 
use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let file = env::args().nth(1).expect("Please supply a filename");

    let text = read_to_string(&file).expect("Bad file man!");

    println!("File had {} bytes", text.len());
}