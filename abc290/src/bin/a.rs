use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let mut ans = 0;

    for t in b {
        ans += a[t - 1];
    }

    println!("{}", ans);
}
