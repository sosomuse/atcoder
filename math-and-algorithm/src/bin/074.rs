use proconio::input;

fn main() {
    input! {
        n: isize,
        a: [isize; n],
    };

    let mut ans = 0;

    for i in 1..=n {
        ans += a[(i as usize) - 1] * (-n + 2 * i - 1);
    }

    println!("{}", ans);
}
