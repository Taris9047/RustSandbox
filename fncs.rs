// Some functions to test out...

// Absolute
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// ensuring a number is withing a range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

// Factorial
fn factorial(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main() {
    let num = -3;
    let mut res = abs(num as f64);
    let min = 1;
    let max = 3;
    let mid = 2;
    println!("abs {}: {}", num, res);
    res = clamp(mid as f64, min as f64, max as f64);
    println!("clamped {}<{}<{}: {}", min, mid, max, res);
    let fac_num = 10;
    let res_u = factorial(fac_num);
    println!("factorial {} is {}", fac_num, res_u);
}
