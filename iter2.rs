// iter2.rs
// Unlike, https://stevedonovan.github.io/rust-gentle-intro/1-basics.html, 
// This program works!!
//
// Yes, we can just iterate arrays like Python!
//
fn main() {
    let arr = [10, 20, 30];
    for i in arr {
        println!("{}", i);
    }
}