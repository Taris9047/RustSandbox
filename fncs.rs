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
fn clamp(x: f64, x1: f64, x2: f64) {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

// Factorial
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1);
    }
}