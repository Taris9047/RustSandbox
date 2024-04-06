// mutation in function
// This is just like C!!
//
fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res = 0.0;
    println!("(b4) res is {}", res);
    modifies(&mut res);
    println!("(l8) res is {}", res);
}