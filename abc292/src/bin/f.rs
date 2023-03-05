use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        mut a: f64,
        mut b: f64,
    }

    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    let mut ok = 0f64;
    let mut ng = PI / 6.0;

    while ng - ok > 1e-12 {
        let mid = (ok + ng) / 2.0;
        let x = a / mid.cos();
        if x * (PI / 6.0 - mid).cos() <= b {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", a / ok.cos());
}
