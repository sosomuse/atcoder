use proconio::input;

const MAX: usize = 1_000_000_000;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };

    let mut ans = 0;
    let mut n = 1;

    while n < MAX {
        if is_buyable(a, b, n, x) {
            ans = n;
        } else {
            break;
        }

        n += 100000;
        n = n.min(MAX);
    }

    for i in ((n.saturating_sub(100000))..=n).rev() {
        if is_buyable(a, b, i, x) {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}

fn is_buyable(a: usize, b: usize, n: usize, x: usize) -> bool {
    let mut digits = 1;
    let mut t = n;
    while t >= 10 {
        t /= 10;
        digits += 1;
    }

    a * n + b * digits <= x
}
