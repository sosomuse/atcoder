use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }

    let rh = ((h / 12.0) + (m / 60.0 / 12.0)) * 2.0 * PI;
    let rm = m / 60.0 * 2.0 * PI;
    let dhm = rm - rh;

    let x2 = a.powf(2.) + b.powf(2.) - 2.0 * a * b * dhm.cos();

    println!("{}", x2.sqrt())
}
