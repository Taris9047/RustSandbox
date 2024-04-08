// Learning Ropes
use std::f64::consts;

fn main() {
    let x = consts::PI*2.0;
    let abs_diff = (x.cos()-1.0).abs();
    println!("|cos(2*Pi)/2| = {}", abs_diff);
    assert!(abs_diff<1e-10);
}