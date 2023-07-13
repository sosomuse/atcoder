use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [usize; n],
        c: [usize; n - 1],
    };

    let mut ans = 0;
    let mut prev = 100000;

    for i in 0..n {
        ans += b[a[i]];

        if prev + 1 == a[i] {
            ans += c[prev];
        }

        prev = a[i];
    }

    println!("{}", ans);
}
