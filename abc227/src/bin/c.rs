use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut a = 1;
    let mut ans = 0;

    while a * a * a <= n {
        let mut b = a;
        while a * b * b <= n {
            let max_c = n / (a * b);
            ans += max_c - b + 1;

            b += 1;
        }

        a += 1;
    }

    println!("{}", ans);
}
