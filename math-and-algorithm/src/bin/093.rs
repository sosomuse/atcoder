use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let (ans, is_overflow) = icm(a, b);

    if is_overflow || ans > 1_000_000_000_000_000_000 {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}

// ユークリッドの互除法（最大公約数）
fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}

// 最小公倍数
fn icm(a: usize, b: usize) -> (usize, bool) {
    return (a / gcd(a, b)).overflowing_mul(b);
}
