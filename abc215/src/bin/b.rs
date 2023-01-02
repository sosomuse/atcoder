use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    let mut ans = 0;

    while n > 1 {
        n /= 2;
        ans += 1;
    }

    println!("{}", ans);
}
