// string5.rs
// Just like Python!
fn main() {
    let mut s = String::new();
    s.push('H');
    println!("s: {:?}", s);
    s.push_str("ello");
    println!("s: {:?}", s);
    s.push(' ');
    println!("s: {:?}", s);
    s += "World!";
    println!("s: {:?}", s);
    s.pop();
    println!("s: {:?}", s);

    assert_eq!(s, "Hello World");
}