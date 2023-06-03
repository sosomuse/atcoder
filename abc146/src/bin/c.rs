use proconio::input;

const MAX: usize = 1_000_000_000;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };

    let mut left = 1;
    let mut right = MAX + 1;

    while left < right {
        let mid = (left + right) / 2;
        if is_buyable(a, b, mid, x) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    println!("{}", left - 1);
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
