use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
    }

    let mut ans = 0;

    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
