use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize,
    };

    let mut ans = n / x;
    if n % x != 0 {
        ans += 1;
    }

    println!("{}", ans * t);
}
