use proconio::input;

fn main() {
    input! {
        n: f64,
    }

    let mut l = 0.0;
    let mut r = 100.0;

    for _ in 0..10000 {
        let m = (l + r) / 2.0;
        if m * m * m + m < n {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
