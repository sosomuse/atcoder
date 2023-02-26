use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; n * 5],
    };

    let mut ans = 0;

    x.sort();

    for i in n..n * 4 {
        ans += x[i];
    }

    println!("{}", ans as f64 / (n * 3) as f64);
}
