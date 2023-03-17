use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        n: usize,
        x0: f64,
        y0: f64,
        xhalf: f64,
        yhalf: f64,
    }

    let (xc, yc) = ((xhalf + x0) / 2.0, (yhalf + y0) / 2.0);
    let rad = 2.0 * PI / (n as f64);
    let x = (x0 - xc) * rad.cos() - (y0 - yc) * rad.sin() + xc;
    let y = (x0 - xc) * rad.sin() + (y0 - yc) * rad.cos() + yc;

    println!("{} {}", x, y);
}
