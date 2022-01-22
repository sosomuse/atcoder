use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        r: f64,
    }
    println!("{}", r * 2.0 * PI);
}
