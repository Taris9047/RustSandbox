// Vector sort!
fn main() {
    let mut v1 = vec![1, 23, 35, 234, 3, 2, 13, 23, 23, 1, 34, 67, 20];
    println!("Before sort: {:?}", v1);
    v1.sort();
    println!("After sort: {:?}", v1);
    v1.dedup();
    println!("After sort+dedup: {:?}", v1);
}