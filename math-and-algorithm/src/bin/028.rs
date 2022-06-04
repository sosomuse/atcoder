use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: f64 = 0.;

    for i in 0..n {
        let v = n - i;
        ans += 1. / (v as f64 / n as f64);
    }

    println!("{}", ans)
}
