// function to test types
fn sqr(x: f64) -> f64 {
    x*x
}

fn main() {
    let basen = 2;
    let res = sqr(basen as f64);
    println!("sqr({}): {}", basen, res);
}