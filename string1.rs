// String or array?
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "Hello, World!"; // string slice
    let s = text.to_string(); // now s is string!

    dump(text);
    dump(&s);
}