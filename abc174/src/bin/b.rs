use proconio::input;

fn main() {
    input! {
        n: i64,
        x: [i64; n],
    }

    let mut m: i64 = 0;
    let mut y: i64 = 0;
    let mut t: i64 = 0;

    for v in &x {
        m += v.abs();
        y += v * v;
        t = t.max(v.abs());
    }

    println!("{}", m);
    println!("{}", (y as f64).sqrt());
    println!("{}", t);
}
