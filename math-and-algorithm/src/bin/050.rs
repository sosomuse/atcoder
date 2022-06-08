use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut ans = 1;

    for _ in 0..b {
        ans = ans * a % 1000000007;
    }

    println!("{}", ans);
}
