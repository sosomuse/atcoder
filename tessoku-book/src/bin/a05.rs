use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize,
    };

    let mut ans = 0;

    for i in 1..=n {
        for j in 1..=n {
            let x = k - i - j;

            if x >= 1 && x <= n {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
